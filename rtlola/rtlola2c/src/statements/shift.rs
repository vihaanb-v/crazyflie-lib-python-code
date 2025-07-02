use rtlola_streamir::{formatter::names::GetStreamName, ir::StreamReference};

use crate::{
    constructs::{FunctionDefinition, RequirementKey, StructDefinition},
    MemoryStruct,
};

pub(crate) struct ShiftFunction(pub(crate) StreamReference);

impl FunctionDefinition for ShiftFunction {
    fn name(&self, f: &crate::CFormatter) -> String {
        format!("shift_{}", f.stream_name(self.0))
    }

    fn body(self, f: &crate::CFormatter) -> String {
        f.static_buffer(self.0).unwrap().shift_code(f)
    }

    fn key(&self) -> crate::constructs::RequirementKey {
        RequirementKey::Shift(self.0)
    }

    fn arguments(&self, f: &crate::CFormatter) -> Vec<crate::constructs::Argument> {
        vec![MemoryStruct.into_argument(f).reference()]
    }

    fn file(&self, f: &crate::CFormatter) -> std::path::PathBuf {
        f.monitor_file()
    }
}
