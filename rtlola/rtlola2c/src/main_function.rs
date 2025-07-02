use std::path::PathBuf;

use clap::ValueEnum;
use itertools::Itertools;
use rtlola_streamir::{
    formatter::{names::GetStreamName, types::TypeFormatter},
    ir::Type,
};

use crate::{
    constructs::{Argument, FunctionDefinition, RequirementKey},
    io::{InternalEvent, PrintVerdict, VerdictStruct},
    memory::InitMemory,
    CFormatter, CType, MemoryStruct, StructDefinition,
};

#[derive(Clone, Debug, ValueEnum, Copy)]
/// The kind of main function to generate
pub enum MainFunction {
    /// No main function
    NoMain,
    /// A main function reading a trace from a CSV file
    CsvOffline,
}

impl MainFunction {
    pub(crate) fn insert_requirement(&self, f: &CFormatter) {
        match self {
            MainFunction::NoMain => {}
            MainFunction::CsvOffline => {
                f.call_function::<_, String>(CsvOffline, &[]);
            }
        }
    }
}

struct ReadField(Type);

impl FunctionDefinition for ReadField {
    fn name(&self, f: &CFormatter) -> String {
        format!("read_{}", f.ty(self.0.clone()))
    }

    fn body(self, _f: &CFormatter) -> String {
        let value_parser = match self.0 {
            Type::Int(i) => format!("(int{i}_t) atoi(token)"),
            Type::UInt(i) => format!("(uint{i}_t) atoi(token)"),
            Type::Float32 => "atof(token)".into(),
            Type::Float64 => "atof(token)".into(),
            Type::Bool => "strcmp(token, \"true\") == 0 ? 1 : 0".into(),
            _ => unreachable!(),
        };
        format!(
            "if (token[0] == \'#\') {{
        *present = 0;
        }} else {{
        *value = {value_parser};
        *present = 1;
        }}"
        )
    }

    fn arguments(&self, _f: &CFormatter) -> Vec<Argument> {
        vec![
            Argument::Normal("token".into(), CType::Char.reference()),
            Argument::Normal("value".into(), CType::Lola(self.0.clone()).reference()),
            Argument::Normal("present".into(), CType::Bool.reference()),
        ]
    }

    fn key(&self) -> RequirementKey {
        RequirementKey::ReadField(self.0.clone())
    }

    fn file(&self, _f: &CFormatter) -> PathBuf {
        ReadEvent.file(_f)
    }
}

struct ReadEvent;

impl FunctionDefinition for ReadEvent {
    fn name(&self, _f: &CFormatter) -> String {
        "read_event".into()
    }

    fn body(self, f: &CFormatter) -> String {
        f.import(self.file(f), "stdio");
        f.import(self.file(f), "stdlib");
        let s = f
            .inputs()
            .enumerate()
            .flat_map(|(idx, i)| {
                [
                    format!(
                        "token = strtok({}, \",\");",
                        if idx == 0 { "line" } else { "NULL" }
                    ),
                    "if (!token) {\nprintf(\"incomplete csv row.\\n\");\nexit(1);\n}".into(),
                    f.call_function_stmt(
                        ReadField(f.stream_ty(i).lola()),
                        &[
                            "token",
                            &format!("&{}->{}", InternalEvent.argument_name(f), f.stream_name(i)),
                            &format!(
                                "&{}->{}",
                                InternalEvent.argument_name(f),
                                f.internal_event_present_flag(i),
                            ),
                        ],
                    ),
                ]
            })
            .join("\n");
        [
            "char line[256];",
            &format!(
                "if (!fgets(line, sizeof(line), {})) return 0;",
                f.file_argument().name()
            ),
            "char* token;",
            &s,
            &Self::read_time(f),
            "return 1;",
        ]
        .join("\n")
    }

    fn arguments(&self, f: &CFormatter) -> Vec<crate::constructs::Argument> {
        vec![
            InternalEvent.into_argument(f).reference(),
            f.file_argument(),
        ]
    }

    fn returns(&self, _f: &CFormatter) -> Option<crate::CType> {
        Some(CType::Bool)
    }

    fn key(&self) -> RequirementKey {
        RequirementKey::ReadEvent
    }

    fn file(&self, _f: &CFormatter) -> PathBuf {
        CsvOffline.file(_f)
    }
}

impl ReadEvent {
    fn read_time(f: &CFormatter) -> String {
        [
            "token = strtok(NULL, \",\");",
            "if (!token) {\nprintf(\"missing time column.\\n\");\nexit(1);\n}",
            "bool present;",
            &f.call_function_stmt(
                ReadField(Type::Float64),
                &[
                    "token",
                    &format!(
                        "&{}->{}",
                        InternalEvent.argument_name(f),
                        f.time_argument_name()
                    ),
                    "&present".into(),
                ],
            ),
            "if (!token) {\nprintf(\"time column can not be #.\\n\");\nexit(1);\n}",
        ]
        .join("\n")
    }
}

struct CsvOffline;

impl FunctionDefinition for CsvOffline {
    fn name(&self, _f: &CFormatter) -> String {
        "main".into()
    }

    fn body(self, f: &CFormatter) -> String {
        [
            "if (argc != 2) {\nprintf(\"Give trace as first an only argument.\\n\");\nexit(1);\n}"
                .into(),
            f.variable_declaration_with_initialization(
                f.file_argument(),
                "fopen(argv[1], \"r\")".into(),
            ),
            f.variable_declaration(MemoryStruct.into_argument(f)),
            f.call_function_stmt(InitMemory, &[MemoryStruct.argument_name_ref(f)]),
            f.print_verdict_header(),
            "char header[256];".into(),
            format!("fgets(header, 256, {});", f.file_argument().name()),
            "while (1) {".into(),
            f.variable_declaration(InternalEvent.into_argument(f)),
            format!(
                "if (!{}) break;",
                f.call_function(
                    ReadEvent,
                    &[
                        format!("&{}", InternalEvent.argument_name(f)),
                        f.file_argument().name().into()
                    ]
                )
            ),
            f.variable_declaration_with_initialization(
                VerdictStruct.into_argument(f),
                format!(
                    "cycle({}, {})",
                    MemoryStruct.argument_name_ref(f),
                    InternalEvent.argument_name(f)
                ),
            ),
            f.call_function_stmt(
                PrintVerdict,
                &[format!("&{}", VerdictStruct.argument_name(f))],
            ),
            "}".into(),
            "return 0;".into(),
        ]
        .join("\n")
    }

    fn arguments(&self, _f: &CFormatter) -> Vec<Argument> {
        vec![
            Argument::Normal("argc".into(), CType::Int),
            Argument::Normal("argv".into(), CType::Char.reference().reference()),
        ]
    }

    fn returns(&self, _f: &CFormatter) -> Option<CType> {
        Some(CType::Int)
    }

    fn key(&self) -> RequirementKey {
        RequirementKey::Main
    }

    fn file(&self, _f: &CFormatter) -> PathBuf {
        _f.monitor_file()
    }
}

impl CFormatter {
    fn file_argument(&self) -> Argument {
        Argument::Normal("f".into(), CType::Other("FILE".into()).reference())
    }

    fn print_verdict_header(&self) -> String {
        let headers = self
            .verdict_streams
            .iter()
            .map(|s| self.stream_name(*s))
            .join(",");
        format!("printf(\"{},time\\n\");", headers)
    }
}
