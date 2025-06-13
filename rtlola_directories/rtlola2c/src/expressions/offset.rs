use std::path::PathBuf;

use rtlola_streamir::ir::StreamReference;

use crate::{
    constructs::{Argument, FunctionDefinition, RequirementKey},
    CFormatter, CType, MemoryStruct, StructDefinition,
};

pub(crate) struct OffsetAccess(pub(crate) StreamReference);

impl FunctionDefinition for OffsetAccess {
    fn name(&self, f: &CFormatter) -> String {
        f.offset_access_function_name(self.0)
    }

    fn body(self, f: &CFormatter) -> String {
        f.static_buffer(self.0).unwrap().get_value(
            f.offset_argument_name(),
            f.default_argument_name(),
            f,
        )
    }

    fn arguments(&self, f: &CFormatter) -> Vec<Argument> {
        vec![
            MemoryStruct.into_argument(f).reference(),
            Argument::Normal(f.offset_argument_name(), CType::Int),
            Argument::Normal(f.default_argument_name(), f.stream_ty(self.0)),
        ]
    }

    fn returns(&self, f: &CFormatter) -> Option<CType> {
        Some(f.stream_ty(self.0))
    }

    fn key(&self) -> RequirementKey {
        RequirementKey::OffsetAccess(self.0)
    }

    fn file(&self, f: &CFormatter) -> PathBuf {
        f.monitor_file()
    }
}
