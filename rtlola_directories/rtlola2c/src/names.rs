use itertools::Itertools;
use rtlola_streamir::{
    formatter::{names::GetStreamName, types::TypeFormatter},
    function_names,
    ir::{StreamReference, Type, WindowReference},
};

use crate::{constructs::Argument, CFormatter, CType};

function_names! {CFormatter,
    internal_event_present_flag(stream): "{stream}_is_present",
    verdict_present_flag(stream): "{stream}_is_present",
    internal_event_input_value(stream): "{stream}",
    dynamic_guard_function_name(num): "expr_{num}",
    sync_access_function_name(stream): "{stream}_sync",
    offset_access_function_name(stream): "{stream}_offset",
    get_access_function_name(stream): "{stream}_get",
    is_fresh_access_function_name(stream): "{stream}_is_fresh",
    spawn_function_name(stream): "spawn_{stream}",
    close_function_name(stream): "close_{stream}",

    alive_argument_name(): "alive",
    values_argument_name(): "values",
    valid_argument_name(): "valid",
    current_argument_name(): "current",
    is_fresh_argument_name(): "is_fresh",
    new_value_argument_name(): "new_value",
    default_argument_name(): "def",
    offset_argument_name(): "offset",

    internal_event_struct_name(): "InternalEvent",
    verdict_struct_name(): "Verdict",
    time_argument_name(): "time",

    build_verdict_function_name(): "build_verdict",
    static_str_constant_name(num): "STR_CONSTANT_{num}"
}

impl GetStreamName for CFormatter {
    fn stream_name(&self, sr: StreamReference) -> String {
        self.sr2memory[&sr].name.to_owned()
    }

    fn window_name(&self, sref: WindowReference) -> String {
        match sref {
            WindowReference::Sliding(i) => format!("sliding{i}"),
            WindowReference::Discrete(i) => format!("discrete{i}"),
            WindowReference::Instance(i) => format!("instance{i}"),
        }
    }
}

impl CFormatter {
    pub(crate) fn alive_argument(&self) -> Argument {
        Argument::Normal(self.alive_argument_name(), CType::Bool)
    }

    pub(crate) fn time_argument(&self) -> Argument {
        Argument::Normal(self.time_argument_name(), CType::Lola(Type::Float64))
    }

    pub(crate) fn tuple_struct_name(&self, ty: &[Type]) -> String {
        format!(
            "TUPLE{}_{}",
            ty.len(),
            ty.iter()
                .map(|ty| self.ty(ty.to_owned()).to_ascii_uppercase())
                .join("__")
        )
    }

    pub(crate) fn tuple_argument_name(&self, i: usize) -> String {
        format!("_{i}")
    }
}
