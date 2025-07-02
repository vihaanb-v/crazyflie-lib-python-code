mod close;
mod eval;
mod input;
mod shift;
mod spawn;

use std::path::PathBuf;

use close::CloseFunction;
use eval::EvalFunction;
use input::InputFunction;
use rtlola_streamir::{
    formatter::{
        guards::GuardFormatter,
        statements::{DefaultStmtFormatter, StmtFormatter},
    },
    ir::{
        expressions::Expr, Guard, InputReference, LocalFreqRef, OutputReference, Stmt,
        StreamReference, WindowReference,
    },
};
use shift::ShiftFunction;
use spawn::SpawnFunction;

use crate::{
    constructs::{Argument, FunctionDefinition, RequirementKey, StructDefinition},
    io::{InternalEvent, NewVerdict, VerdictStruct},
    memory::ClearActivation,
    CFormatter, MemoryStruct,
};

pub(crate) struct CycleFunction(pub Stmt);

impl FunctionDefinition for CycleFunction {
    fn name(&self, _f: &crate::CFormatter) -> String {
        "cycle".into()
    }

    fn body(self, f: &crate::CFormatter) -> String {
        [
            format!(
                "{}->{} = {}.{};",
                MemoryStruct.argument_name(f),
                f.time_argument_name(),
                InternalEvent.argument_name(f),
                f.time_argument_name()
            ),
            f.stmt(self.0),
            f.variable_declaration_with_initialization(
                VerdictStruct.into_argument(f),
                f.call_function(NewVerdict, &[MemoryStruct.argument_name(f)]),
            ),
            f.call_function_stmt(ClearActivation, &[MemoryStruct.argument_name(f)]),
            format!("return {};", VerdictStruct.argument_name(f)),
        ]
        .join("\n")
    }

    fn key(&self) -> crate::constructs::RequirementKey {
        RequirementKey::CycleFunction
    }

    fn file(&self, _f: &crate::CFormatter) -> PathBuf {
        _f.monitor_file()
    }

    fn header_file(&self, _f: &crate::CFormatter) -> Option<(RequirementKey, PathBuf)> {
        Some((RequirementKey::CycleHeader, _f.header_file()))
    }

    fn arguments(&self, _f: &CFormatter) -> Vec<Argument> {
        vec![
            MemoryStruct.into_argument(_f).reference(),
            InternalEvent.into_argument(_f),
        ]
    }

    fn returns(&self, _f: &CFormatter) -> Option<crate::CType> {
        Some(VerdictStruct.as_ty(_f))
    }
}

impl DefaultStmtFormatter for CFormatter {
    fn shift(&self, sr: StreamReference) -> String {
        let shift = ShiftFunction(sr);
        self.call_function_stmt(shift, &[MemoryStruct.argument_name(self)])
    }

    fn input(&self, sr: InputReference) -> String {
        let new_value = format!(
            "{}.{}",
            InternalEvent.argument_name(self),
            self.internal_event_input_value(StreamReference::In(sr))
        );
        self.call_function_stmt(
            InputFunction(sr),
            &[MemoryStruct.argument_name(self), new_value],
        )
    }

    fn spawn(
        &self,
        sr: OutputReference,
        with: Option<Vec<Expr>>,
        local_frequencies: Vec<LocalFreqRef>,
        windows: Vec<WindowReference>,
    ) -> String {
        assert!(local_frequencies.is_empty());
        assert!(windows.is_empty());
        if let Some(_with) = with {
            unimplemented!()
        } else {
            format!(
                "if (!{}) {}",
                self.alive(sr.sr()),
                self.call_function_stmt(SpawnFunction(sr), &[MemoryStruct.argument_name(self)])
            )
        }
    }

    fn eval(&self, sr: OutputReference, with: Expr, idx: usize) -> String {
        let eval = EvalFunction {
            sr,
            expr: with,
            i: idx,
        };
        self.call_function_stmt(eval, &[MemoryStruct.argument_name(self)])
    }

    fn close(
        &self,
        sr: OutputReference,
        _local_frequencies: Vec<LocalFreqRef>,
        _windows: Vec<WindowReference>,
    ) -> String {
        self.call_function_stmt(CloseFunction(sr), &[MemoryStruct.argument_name(self)])
    }

    fn r#if(&self, guard: Guard, cons: Stmt, alt: Option<Stmt>) -> String {
        let guard = self.guard(guard);
        let cons = self.stmt(cons);
        if let Some(alt) = alt {
            let alt = self.stmt(alt);
            format!("if ({guard}) {{\n{cons}\n}} else {{\n{alt}\n}}")
        } else {
            format!("if ({guard}) {{\n{cons}\n}}")
        }
    }

    fn iterate(&self, _sr: Vec<OutputReference>, _inner: Stmt) -> String {
        todo!()
    }

    fn assign(
        &self,
        _sr: Vec<OutputReference>,
        _parameter_expr: Vec<Expr>,
        _inner: Stmt,
    ) -> String {
        todo!()
    }
}
