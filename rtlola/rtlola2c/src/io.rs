use std::{fmt::Write, path::PathBuf};

use itertools::Itertools;
use rtlola_streamir::{formatter::names::GetStreamName, ir::Type};

use crate::{
    constructs::{Argument, FunctionDefinition, RequirementKey, StructDefinition},
    expressions::{is_fresh::IsFresh, sync::SyncAccess},
    CFormatter, CType, MemoryStruct,
};

pub(crate) struct InternalEvent;

impl StructDefinition for InternalEvent {
    fn key(&self) -> RequirementKey {
        RequirementKey::InternalEventStruct
    }

    fn struct_name(&self, f: &crate::CFormatter) -> String {
        f.internal_event_struct_name()
    }

    fn fields(&self, f: &crate::CFormatter) -> Vec<Argument> {
        f.inputs()
            .flat_map(|i| {
                [
                    Argument::Normal(f.stream_name(i).to_owned(), f.stream_ty(i)),
                    Argument::Normal(f.internal_event_present_flag(i), CType::Bool),
                ]
            })
            .chain(Some(f.time_argument()))
            .collect()
    }

    fn file(&self, f: &crate::CFormatter) -> std::path::PathBuf {
        f.header_file()
    }
}

pub(crate) struct VerdictStruct;

impl StructDefinition for VerdictStruct {
    fn key(&self) -> RequirementKey {
        RequirementKey::VerdictStruct
    }

    fn file(&self, f: &crate::CFormatter) -> std::path::PathBuf {
        f.header_file()
    }

    fn struct_name(&self, f: &crate::CFormatter) -> String {
        f.verdict_struct_name()
    }

    fn fields(&self, f: &crate::CFormatter) -> Vec<Argument> {
        f.verdict_streams
            .iter()
            .copied()
            .flat_map(|o| {
                [
                    Argument::Normal(f.stream_name(o).to_owned(), f.stream_ty(o)),
                    Argument::Normal(f.verdict_present_flag(o), CType::Bool),
                ]
            })
            .chain(Some(f.time_argument()))
            .collect()
    }
}

pub(crate) struct NewVerdict;

impl FunctionDefinition for NewVerdict {
    fn name(&self, f: &crate::CFormatter) -> String {
        f.build_verdict_function_name()
    }

    fn body(self, f: &crate::CFormatter) -> String {
        f.import(self.file(f), "string");
        [
            f.variable_declaration(VerdictStruct.into_argument(f)),
            format!(
                "memset(&{}, 0, sizeof({}));",
                VerdictStruct.argument_name(f),
                VerdictStruct.argument_name(f)
            ),
            f.verdict_streams.iter().copied()
                .map(|o| {
                    format!(
                        "if ({is_fresh}) {{\n{struct}.{value} = {sync};{struct}.{is_present} = 1;\n}}",
                        is_fresh=f.call_function(IsFresh(o), &[MemoryStruct.argument_name(f)]),
                        sync=f.call_function(SyncAccess(o), &[MemoryStruct.argument_name(f)]),
                        struct=VerdictStruct.argument_name(f),
                        value=f.stream_name(o),
                        is_present=f.verdict_present_flag(o)
                    )
                })
                .join("\n"),
            format!("{}.{} = {}->{};", VerdictStruct.argument_name(f), f.time_argument_name(), MemoryStruct.argument_name(f), f.time_argument_name()),
            format!("return {};", VerdictStruct.argument_name(f)),
        ]
        .join("\n")
    }

    fn key(&self) -> RequirementKey {
        RequirementKey::NewVerdict
    }

    fn arguments(&self, f: &crate::CFormatter) -> Vec<Argument> {
        vec![MemoryStruct.into_argument(f).reference()]
    }

    fn returns(&self, f: &crate::CFormatter) -> Option<CType> {
        Some(VerdictStruct.as_ty(f))
    }

    fn file(&self, f: &crate::CFormatter) -> std::path::PathBuf {
        f.monitor_file()
    }
}

impl CFormatter {
    pub(crate) fn format_string(ty: Type) -> String {
        match ty {
            Type::Int(_) | Type::UInt(_) => "%d".into(),
            Type::Bool => "%s".into(),
            Type::String => "\\\"%s\\\"".into(),
            Type::Float32 => "%f".into(),
            Type::Float64 => "%lf".into(),
            Type::Fixed(_) | Type::UFixed(_) => unimplemented!(),
            Type::Tuple(inner) => {
                format!("({})", inner.into_iter().map(Self::format_string).join(","))
            }
            Type::Bytes => unimplemented!(),
            Type::Option(_) => unreachable!(),
        }
    }

    fn unroll_tuple_elements(&self, ty: Type, cur: String) -> Vec<String> {
        match ty {
            Type::Tuple(items) => items
                .into_iter()
                .enumerate()
                .flat_map(|(i, ty)| {
                    let new_cur = format!("{cur}.{}", self.tuple_argument_name(i));
                    self.unroll_tuple_elements(ty, new_cur)
                })
                .collect(),
            Type::Bool => vec![format!("({cur}) ? \"true\" : \"false\"")],
            _ => vec![cur],
        }
    }
}

pub(crate) struct PrintVerdict;

impl FunctionDefinition for PrintVerdict {
    fn name(&self, _f: &CFormatter) -> String {
        "print_verdict".into()
    }

    fn arguments(&self, f: &CFormatter) -> Vec<Argument> {
        vec![VerdictStruct.into_argument(f).reference()]
    }

    fn body(self, f: &CFormatter) -> String {
        let verdict = VerdictStruct.argument_name(f);
        let i = f
            .verdict_streams
            .iter()
            .copied()
            .fold(String::new(), |mut s, o| {
                let values = f
                    .unroll_tuple_elements(
                        f.stream_ty(o).lola(),
                        format!("{verdict}->{}", f.stream_name(o),),
                    )
                    .join(", ");
                write!(
                    &mut s,
                    "if ({verdict}->{has_value}) {{\n\
    printf(\"{format_string},\", {values});\n\
    }} else {{\n\
     printf(\"#,\");\n\
    }}",
                    has_value = f.verdict_present_flag(o),
                    format_string = CFormatter::format_string(f.stream_ty(o).lola()),
                )
                .unwrap();
                s
            });
        format!(
            "{i}\nprintf(\"%f\\n\", {verdict}->{time});",
            time = f.time_argument_name()
        )
    }

    fn key(&self) -> RequirementKey {
        RequirementKey::PrintVerdict
    }

    fn file(&self, f: &CFormatter) -> PathBuf {
        f.monitor_file()
    }

    fn header_file(&self, f: &CFormatter) -> Option<(RequirementKey, PathBuf)> {
        Some((RequirementKey::PrintVerdictHeader, f.header_file()))
    }
}
