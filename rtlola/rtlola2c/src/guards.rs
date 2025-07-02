use std::time::Duration;

use rtlola_streamir::{
    formatter::{expressions::ExprFormatter, guards::DefaultGuardFormatter},
    ir::{expressions::Expr, LocalFreqRef, StreamReference},
};

use crate::{
    constructs::{FunctionDefinition, RequirementKey, StructDefinition},
    io::InternalEvent,
    CFormatter, CType, MemoryStruct,
};

impl DefaultGuardFormatter for CFormatter {
    fn stream(&self, sr: StreamReference) -> String {
        format!(
            "{}.{}",
            InternalEvent.argument_name(self),
            self.internal_event_present_flag(sr)
        )
    }

    fn alive(&self, sr: StreamReference) -> String {
        format!(
            "{}->{}.{}",
            MemoryStruct.argument_name(self),
            self.dynamic_memory_struct(sr).unwrap().argument_name(self),
            self.alive_argument_name()
        )
    }

    fn dynamic(&self, expr: Expr) -> String {
        self.call_function(
            DynamicGuard::new(expr, self),
            &[MemoryStruct.argument_name(self)],
        )
    }

    fn global_freq(&self, _duration: Duration) -> String {
        unimplemented!()
    }

    fn local_freq(&self, _freq_ref: LocalFreqRef) -> String {
        unimplemented!()
    }

    fn constant(&self, b: bool) -> String {
        match b {
            true => "true",
            false => "false",
        }
        .into()
    }
}

struct DynamicGuard(Expr, usize);

impl DynamicGuard {
    fn new(expr: Expr, f: &CFormatter) -> Self {
        let mut expr_counter = f.expr_counter.lock().unwrap();
        let params = expr.contains_parameter_access();
        if let Some(c) = expr_counter.get(&(expr.clone(), params)) {
            Self(expr, *c)
        } else {
            let mut num_exprs = f.num_exprs.lock().unwrap();
            let c = *num_exprs;
            expr_counter.insert((expr.clone(), params), c);
            *num_exprs += 1;
            Self(expr, c)
        }
    }
}

impl FunctionDefinition for DynamicGuard {
    fn name(&self, f: &CFormatter) -> String {
        f.dynamic_guard_function_name(self.1)
    }

    fn body(self, f: &CFormatter) -> String {
        format!("return {};", f.expr(self.0))
    }

    fn key(&self) -> RequirementKey {
        RequirementKey::DynamicGuard(self.1)
    }

    fn returns(&self, _f: &CFormatter) -> Option<CType> {
        Some(crate::CType::Bool)
    }

    fn arguments(&self, f: &CFormatter) -> Vec<crate::constructs::Argument> {
        vec![MemoryStruct.into_argument(f).reference()]
    }

    fn file(&self, f: &CFormatter) -> std::path::PathBuf {
        f.monitor_file()
    }
}
