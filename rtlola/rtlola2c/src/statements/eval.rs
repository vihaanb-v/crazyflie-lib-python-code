use rtlola_streamir::{
    formatter::{expressions::ExprFormatter, names::GetStreamName},
    ir::{expressions::Expr, OutputReference},
};

use crate::{
    constructs::{Argument, FunctionDefinition, RequirementKey, StructDefinition},
    CType, MemoryStruct,
};

pub(crate) struct EvalFunction {
    pub(crate) sr: OutputReference,
    pub(crate) expr: Expr,
    pub(crate) i: usize,
}

impl FunctionDefinition for EvalFunction {
    fn name(&self, f: &crate::CFormatter) -> String {
        format!("eval_{}_{}", f.stream_name(self.sr.sr()), self.i)
    }

    fn body(self, f: &crate::CFormatter) -> String {
        [
            f.variable_declaration_with_initialization(
                Argument::Normal("new_value".into(), CType::Lola(self.expr.ty.clone())),
                f.expr(self.expr.clone()),
            ),
            f.static_buffer(self.sr.sr())
                .unwrap()
                .push_value("new_value".into(), f),
        ]
        .join("\n")
    }

    fn key(&self) -> RequirementKey {
        RequirementKey::Eval(self.sr, self.i)
    }

    fn arguments(&self, f: &crate::CFormatter) -> Vec<crate::constructs::Argument> {
        vec![MemoryStruct.into_argument(f).reference()]
    }

    fn file(&self, f: &crate::CFormatter) -> std::path::PathBuf {
        f.monitor_file()
    }
}
