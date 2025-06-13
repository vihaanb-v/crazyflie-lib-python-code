use rtlola_streamir::{
    formatter::names::GetStreamName,
    ir::{InputReference, StreamReference},
};

use crate::{
    constructs::{FunctionDefinition, RequirementKey, StructDefinition},
    MemoryStruct,
};

pub(crate) struct InputFunction(pub(crate) InputReference);

impl FunctionDefinition for InputFunction {
    fn name(&self, f: &crate::CFormatter) -> String {
        format!("input_{}", f.stream_name(StreamReference::In(self.0)))
    }

    fn body(self, f: &crate::CFormatter) -> String {
        f.static_buffer(StreamReference::In(self.0))
            .unwrap()
            .push_value(f.new_value_argument_name(), f)
    }

    fn key(&self) -> crate::constructs::RequirementKey {
        RequirementKey::Input(self.0)
    }

    fn arguments(&self, f: &crate::CFormatter) -> Vec<crate::constructs::Argument> {
        vec![
            MemoryStruct.into_argument(f).reference(),
            f.stream_ty(StreamReference::In(self.0))
                .argument(f.new_value_argument_name()),
        ]
    }

    fn file(&self, f: &crate::CFormatter) -> std::path::PathBuf {
        f.monitor_file()
    }
}
