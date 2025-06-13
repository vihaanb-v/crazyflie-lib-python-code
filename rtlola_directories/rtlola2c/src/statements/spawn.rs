use rtlola_streamir::ir::OutputReference;

use crate::{
    constructs::{FunctionDefinition, RequirementKey, StructDefinition},
    MemoryStruct,
};

pub(crate) struct SpawnFunction(pub(crate) OutputReference);

impl FunctionDefinition for SpawnFunction {
    fn name(&self, f: &crate::CFormatter) -> String {
        f.spawn_function_name(self.0.sr())
    }

    fn body(self, f: &crate::CFormatter) -> String {
        f.import(self.file(f), "string");
        format!(
            "memset(&{memory}->{buffer}, 0, sizeof({memory}->{buffer}));\n{memory}->{buffer}.{alive} = true;",
            memory = MemoryStruct.argument_name(f),
            buffer = f
                .dynamic_memory_struct(self.0.sr())
                .unwrap()
                .argument_name(f),
            alive = f.alive_argument_name()
        )
    }

    fn arguments(&self, f: &crate::CFormatter) -> Vec<crate::constructs::Argument> {
        vec![MemoryStruct.into_argument(f).reference()]
    }

    fn key(&self) -> crate::constructs::RequirementKey {
        RequirementKey::Spawn(self.0)
    }

    fn file(&self, f: &crate::CFormatter) -> std::path::PathBuf {
        f.monitor_file()
    }
}
