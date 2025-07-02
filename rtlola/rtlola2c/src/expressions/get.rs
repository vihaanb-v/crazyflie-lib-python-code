use std::path::PathBuf;

use crate::{
    constructs::{Argument, FunctionDefinition, RequirementKey},
    CFormatter, CType, MemoryStruct, StructDefinition,
};
use rtlola_streamir::ir::StreamReference;

pub(crate) struct GetAccess(pub(crate) StreamReference);

impl FunctionDefinition for GetAccess {
    fn name(&self, f: &CFormatter) -> String {
        f.get_access_function_name(self.0)
    }

    fn body(self, f: &CFormatter) -> String {
        format!(
            "if ({}) return {}; else return {};",
            f.static_buffer(self.0).unwrap().is_fresh(f),
            f.static_buffer(self.0).unwrap().sync_access(f),
            f.default_argument_name()
        )
    }

    fn key(&self) -> RequirementKey {
        RequirementKey::GetAccess(self.0)
    }

    fn arguments(&self, f: &CFormatter) -> Vec<Argument> {
        vec![
            MemoryStruct.into_argument(f).reference(),
            Argument::Normal(f.default_argument_name(), f.stream_ty(self.0)),
        ]
    }

    fn returns(&self, f: &CFormatter) -> Option<CType> {
        Some(f.stream_ty(self.0))
    }

    fn file(&self, f: &CFormatter) -> PathBuf {
        f.monitor_file()
    }
}
