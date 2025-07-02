use std::path::PathBuf;

use rtlola_streamir::ir::StreamReference;

use crate::{
    constructs::{FunctionDefinition, RequirementKey},
    CType, MemoryStruct, StructDefinition,
};

pub(crate) struct IsFresh(pub(crate) StreamReference);

impl FunctionDefinition for IsFresh {
    fn name(&self, f: &crate::CFormatter) -> String {
        f.is_fresh_access_function_name(self.0)
    }

    fn body(self, f: &crate::CFormatter) -> String {
        format!("return {};", f.static_buffer(self.0).unwrap().is_fresh(f))
    }

    fn key(&self) -> RequirementKey {
        RequirementKey::IsFreshAccess(self.0)
    }

    fn arguments(&self, f: &crate::CFormatter) -> Vec<crate::constructs::Argument> {
        vec![MemoryStruct.into_argument(f).reference()]
    }

    fn returns(&self, _f: &crate::CFormatter) -> Option<crate::CType> {
        Some(CType::Bool)
    }

    fn file(&self, f: &crate::CFormatter) -> PathBuf {
        f.monitor_file()
    }
}
