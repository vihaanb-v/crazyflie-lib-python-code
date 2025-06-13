use rtlola_streamir::ir::StreamReference;

use crate::{
    constructs::{FunctionDefinition, RequirementKey, StructDefinition},
    CType, MemoryStruct,
};

pub(crate) struct SyncAccess(pub StreamReference);

impl FunctionDefinition for SyncAccess {
    fn name(&self, f: &crate::CFormatter) -> String {
        f.sync_access_function_name(self.0)
    }

    fn body(self, f: &crate::CFormatter) -> String {
        format!(
            "return {};",
            f.static_buffer(self.0).unwrap().sync_access(f)
        )
    }

    fn key(&self) -> RequirementKey {
        RequirementKey::SyncAccess(self.0)
    }

    fn returns(&self, f: &crate::CFormatter) -> Option<CType> {
        Some(f.stream_ty(self.0))
    }

    fn arguments(&self, f: &crate::CFormatter) -> Vec<crate::constructs::Argument> {
        vec![MemoryStruct.into_argument(f).reference()]
    }

    fn file(&self, f: &crate::CFormatter) -> std::path::PathBuf {
        f.monitor_file()
    }
}
