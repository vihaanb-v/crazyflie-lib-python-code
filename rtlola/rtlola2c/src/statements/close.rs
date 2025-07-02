use std::path::PathBuf;

use rtlola_streamir::ir::OutputReference;

use crate::{
    constructs::{FunctionDefinition, RequirementKey},
    CFormatter, MemoryStruct, StructDefinition,
};

pub(crate) struct CloseFunction(pub(crate) OutputReference);

impl FunctionDefinition for CloseFunction {
    fn name(&self, f: &CFormatter) -> String {
        f.close_function_name(self.0.sr())
    }

    fn body(self, f: &CFormatter) -> String {
        // for hold/offset accesses to correctly return the default value, set the valid flags to false
        format!(
            "memset({static_buffer}.{valid}, 0, sizeof({static_buffer}.{valid}));{memory}->{buffer}.{alive} = false;",
            memory = MemoryStruct.argument_name(f),
			static_buffer = f.buffer(self.0.sr()).unwrap(),
            buffer = f
                .dynamic_memory_struct(self.0.sr())
                .unwrap()
                .argument_name(f),
            alive = f.alive_argument_name(),
			valid = f.valid_argument_name()
        )
    }

    fn key(&self) -> RequirementKey {
        RequirementKey::Close(self.0)
    }

    fn arguments(&self, f: &CFormatter) -> Vec<crate::constructs::Argument> {
        vec![MemoryStruct.into_argument(f).reference()]
    }

    fn file(&self, f: &crate::CFormatter) -> PathBuf {
        f.monitor_file()
    }
}
