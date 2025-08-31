use rtlola_streamir::{
    formatter::{
        files::{FilesFormatter, Requirement},
        names::GetStreamName,
    },
    ir::StreamReference,
};

use crate::{constructs::RequirementKey, RustFormatter};

pub(crate) struct MonitorError;

impl MonitorError {
    pub(crate) fn add_requirement(f: &RustFormatter) {
        f.add_requirement(MonitorError);
    }

    pub(crate) fn instance_not_found(
        stream: StreamReference,
        instance: &str,
        f: &RustFormatter,
    ) -> String {
        Self::add_requirement(f);
        let stream_name = f.stream_name(stream);
        if f.no_std_info.is_some() {
            format!(
                "{}::InstanceNotFound {{ stream: \"{stream_name}\" }}",
                f.error_name()
            )
        } else {
            format!("{}::InstanceNotFound {{ stream: \"{stream_name}\", instance: format!(\"{{:?}}\", ({instance})) }}", f.error_name())
        }
    }

    pub(crate) fn too_many_deadlines(f: &RustFormatter) -> String {
        format!("{}::TooManyDeadlines", f.error_name())
    }
}

impl Requirement<RustFormatter> for MonitorError {
    fn key(&self) -> <RustFormatter as FilesFormatter>::Key {
        RequirementKey::MonitorError
    }

    fn format(self, formatter: &RustFormatter) -> String {
        let too_many_instances = if formatter
            .no_std_info
            .as_ref()
            .is_some_and(|i| !i.max_instances.is_empty())
        {
            "\nTooManyInstances,"
        } else {
            ""
        };
        let too_many_deadlines = if formatter.no_std_info.as_ref().is_some() {
            "\nTooManyDeadlines,"
        } else {
            ""
        };
        let instance_not_found = if formatter.no_std_info.is_some() {
            "InstanceNotFound { stream: &'static str },"
        } else {
            "InstanceNotFound { stream: &'static str, instance: String },"
        };
        format!(
            "#[derive(Debug, Clone)]
			pub enum {} {{
            {instance_not_found}
			OutOfBoundsAccess {{ accessed_offset: usize, buffer_size: usize }},{too_many_instances}{too_many_deadlines}
		}}",
            formatter.error_name()
        )
    }

    fn file(&self, formatter: &RustFormatter) -> std::path::PathBuf {
        formatter.main_file()
    }
}
