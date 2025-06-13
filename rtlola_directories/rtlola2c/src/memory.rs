use itertools::Itertools;
use rtlola_streamir::{
    formatter::names::GetStreamName,
    ir::{
        memory::{StreamBuffer, StreamMemory},
        StreamReference,
    },
};

use crate::{
    constructs::{Argument, FunctionDefinition, RequirementKey, StructDefinition},
    CFormatter, CType,
};

pub(crate) struct MemoryStruct;

impl StructDefinition for MemoryStruct {
    fn key(&self) -> RequirementKey {
        RequirementKey::MemoryStruct
    }

    fn file(&self, f: &crate::CFormatter) -> std::path::PathBuf {
        f.header_file()
    }

    fn struct_name(&self, _f: &crate::CFormatter) -> String {
        "Memory".into()
    }

    fn fields(&self, f: &crate::CFormatter) -> Vec<Argument> {
        f.streams()
            .flat_map(|sr| match &f.sr2memory[&sr].buffer {
                StreamMemory::NoMemory => None,
                StreamMemory::Static(buffer) => {
                    Some(f.static_stream_memory_struct(sr, buffer).as_argument(f))
                }
                StreamMemory::Dynamic { .. } => {
                    Some(f.dynamic_memory_struct(sr).unwrap().into_argument(f))
                }
                StreamMemory::Instances { .. } => todo!(),
            })
            .chain(Some(f.time_argument()))
            .collect()
    }
}

impl CFormatter {
    pub(crate) fn static_buffer(&self, sr: StreamReference) -> Option<Box<dyn StaticBufferTrait>> {
        match &self.sr2memory[&sr].buffer {
            StreamMemory::NoMemory => None,
            StreamMemory::Static(buffer) | StreamMemory::Dynamic { buffer, .. } => {
                Some(self.static_stream_memory_struct(sr, buffer))
            }
            StreamMemory::Instances { .. } => unimplemented!(),
        }
    }

    pub(crate) fn dynamic_memory_struct(&self, sr: StreamReference) -> Option<DynamicBuffer> {
        match &self.sr2memory[&sr].buffer {
            &StreamMemory::Dynamic { buffer, .. } => Some(DynamicBuffer(sr, buffer)),
            _ => None,
        }
    }

    pub(crate) fn static_stream_memory_struct(
        &self,
        sr: StreamReference,
        buffer: &StreamBuffer,
    ) -> Box<dyn StaticBufferTrait> {
        match buffer {
            StreamBuffer::SingleValue => Box::new(SingleValueBuffer(sr)),
            StreamBuffer::Bounded(b) => Box::new(BoundedBuffer(sr, *b)),
            StreamBuffer::UnBounded => unimplemented!("not supported in RTLola"),
        }
    }
}

pub(crate) trait StaticBufferTrait {
    fn shift_code(&self, f: &CFormatter) -> String;
    fn push_value(&self, v: String, f: &CFormatter) -> String;
    fn get_value(&self, offset: String, default: String, f: &CFormatter) -> String;
    fn is_fresh(&self, f: &CFormatter) -> String;
    fn reset_fresh(&self, f: &CFormatter) -> String;
    fn sync_access(&self, f: &CFormatter) -> String;

    fn as_argument(&self, f: &CFormatter) -> Argument;
}

impl CFormatter {
    pub(crate) fn buffer(&self, sr: StreamReference) -> Option<String> {
        match &self.sr2memory[&sr].buffer {
            StreamMemory::NoMemory => None,
            StreamMemory::Static(buffer) => Some(format!(
                "{}->{}",
                MemoryStruct.argument_name(self),
                self.static_stream_memory_struct(sr, buffer)
                    .as_argument(self)
                    .name()
            )),
            StreamMemory::Dynamic { .. } => Some(format!(
                "{}->{}.{}",
                MemoryStruct.argument_name(self),
                self.dynamic_memory_struct(sr)
                    .unwrap()
                    .into_argument(self)
                    .name(),
                self.static_buffer(sr).unwrap().as_argument(self).name()
            )),
            StreamMemory::Instances { .. } => todo!(),
        }
    }
}

#[derive(Clone)]
pub(crate) struct SingleValueBuffer(StreamReference);

impl StructDefinition for SingleValueBuffer {
    fn key(&self) -> RequirementKey {
        RequirementKey::StaticStreamMemory(self.0)
    }

    fn file(&self, f: &CFormatter) -> std::path::PathBuf {
        f.header_file()
    }

    fn struct_name(&self, f: &CFormatter) -> String {
        format!("SingleValueBuffer_{}", f.stream_name(self.0))
    }

    fn fields(&self, f: &CFormatter) -> Vec<Argument> {
        vec![
            Argument::Normal(f.values_argument_name(), f.stream_ty(self.0)),
            Argument::Normal(f.valid_argument_name(), CType::Bool),
            Argument::Normal(f.is_fresh_argument_name(), CType::Bool),
        ]
    }
}

impl StaticBufferTrait for SingleValueBuffer {
    fn shift_code(&self, _f: &CFormatter) -> String {
        unreachable!("shifts should be optimized away for single value buffers")
    }

    fn push_value(&self, v: String, f: &CFormatter) -> String {
        format!(
            "{buffer}.{value} = {v};\n{buffer}.{valid} = true;",
            value = f.values_argument_name(),
            valid = f.valid_argument_name(),
            buffer = f.buffer(self.0).unwrap()
        )
    }

    fn get_value(&self, offset: String, default: String, f: &CFormatter) -> String {
        format!(
            "assert({offset} == 0);\nif ({buffer}.{valid})\nreturn {buffer}.{value};\nelse\nreturn {default};",
            buffer=f.buffer(self.0).unwrap(),
            valid = f.valid_argument_name(),
            value = f.values_argument_name()
        )
    }

    fn is_fresh(&self, f: &CFormatter) -> String {
        format!(
            "{buffer}.{is_fresh}",
            buffer = f.buffer(self.0).unwrap(),
            is_fresh = f.is_fresh_argument_name()
        )
    }

    fn reset_fresh(&self, f: &CFormatter) -> String {
        format!(
            "{buffer}.{is_fresh} = 0;",
            buffer = f.buffer(self.0).unwrap(),
            is_fresh = f.is_fresh_argument_name()
        )
    }

    fn sync_access(&self, _f: &CFormatter) -> String {
        format!("{buffer}.value", buffer = _f.buffer(self.0).unwrap())
    }

    fn as_argument(&self, f: &CFormatter) -> Argument {
        <Self as StructDefinition>::into_argument(self.to_owned(), f)
    }
}

#[derive(Clone)]
pub(crate) struct BoundedBuffer(StreamReference, usize);

impl StructDefinition for BoundedBuffer {
    fn key(&self) -> RequirementKey {
        RequirementKey::StaticStreamMemory(self.0)
    }

    fn file(&self, f: &CFormatter) -> std::path::PathBuf {
        f.header_file()
    }

    fn struct_name(&self, f: &CFormatter) -> String {
        format!("BoundedBuffer_{}", f.stream_name(self.0))
    }

    fn fields(&self, f: &CFormatter) -> Vec<Argument> {
        vec![
            Argument::Array(f.values_argument_name(), f.stream_ty(self.0), self.1),
            Argument::Array(f.valid_argument_name(), CType::Bool, self.1),
            Argument::Normal(f.current_argument_name(), CType::Int),
            Argument::Normal(f.is_fresh_argument_name(), CType::Bool),
        ]
    }
}

impl StaticBufferTrait for BoundedBuffer {
    fn shift_code(&self, f: &CFormatter) -> String {
        format!(
            "{buffer}.{current} = ({buffer}.{current} + 1) % {size};",
            size = self.1,
            current = f.current_argument_name(),
            buffer = f.buffer(self.0).unwrap()
        )
    }

    fn push_value(&self, v: String, f: &CFormatter) -> String {
        format!(
            "{buffer}.{values}[{buffer}.{current}] = {v};\n\
            {buffer}.{valid}[{buffer}.{current}] = 1;\n\
            {buffer}.{fresh} = 1;",
            valid = f.valid_argument_name(),
            current = f.current_argument_name(),
            values = f.values_argument_name(),
            buffer = f.buffer(self.0).unwrap(),
            fresh = f.is_fresh_argument_name()
        )
    }

    fn get_value(&self, offset: String, default: String, f: &CFormatter) -> String {
        format!(
            "int i = ({buffer}.{current} - {offset} + {size}) % {size};\nif ({buffer}.{valid}[i])\nreturn {buffer}.{values}[i];\nelse\nreturn {default};",
            size=self.1,
            current = f.current_argument_name(),
            values = f.values_argument_name(),
            valid = f.valid_argument_name(),
            buffer = f.buffer(self.0).unwrap()
        )
    }

    fn is_fresh(&self, f: &CFormatter) -> String {
        format!(
            "{buffer}.{is_fresh}",
            buffer = f.buffer(self.0).unwrap(),
            is_fresh = f.is_fresh_argument_name()
        )
    }

    fn reset_fresh(&self, f: &CFormatter) -> String {
        format!(
            "{buffer}.{is_fresh} = 0;",
            buffer = f.buffer(self.0).unwrap(),
            is_fresh = f.is_fresh_argument_name()
        )
    }

    fn sync_access(&self, f: &CFormatter) -> String {
        format!(
            "{buffer}.{values}[{buffer}.{current}]",
            buffer = f.buffer(self.0).unwrap(),
            current = f.current_argument_name(),
            values = f.values_argument_name()
        )
    }

    fn as_argument(&self, f: &CFormatter) -> Argument {
        <Self as StructDefinition>::into_argument(self.to_owned(), f)
    }
}

#[derive(Clone)]
pub(crate) struct DynamicBuffer(pub(crate) StreamReference, pub(crate) StreamBuffer);

impl StructDefinition for DynamicBuffer {
    fn key(&self) -> RequirementKey {
        RequirementKey::DynamicStreamMemory(self.0)
    }

    fn file(&self, f: &CFormatter) -> std::path::PathBuf {
        f.header_file()
    }

    fn struct_name(&self, f: &CFormatter) -> String {
        format!("DynamicBuffer_{}", f.stream_name(self.0))
    }

    fn fields(&self, f: &CFormatter) -> Vec<Argument> {
        vec![
            f.static_stream_memory_struct(self.0, &self.1)
                .as_argument(f),
            f.alive_argument(),
        ]
    }
}

pub(crate) struct InitMemory;

impl FunctionDefinition for InitMemory {
    fn name(&self, _f: &CFormatter) -> String {
        "init_memory".into()
    }

    fn body(self, f: &CFormatter) -> String {
        f.import(self.file(f), "string");
        format!(
            "memset({mem}, 0, sizeof(*{mem}));",
            mem = MemoryStruct.argument_name(f)
        )
    }

    fn key(&self) -> RequirementKey {
        RequirementKey::InitMemory
    }

    fn file(&self, _f: &CFormatter) -> std::path::PathBuf {
        _f.monitor_file()
    }

    fn arguments(&self, _f: &CFormatter) -> Vec<Argument> {
        vec![MemoryStruct.into_argument(_f).reference()]
    }

    fn header_file(&self, _f: &CFormatter) -> Option<(RequirementKey, std::path::PathBuf)> {
        Some((RequirementKey::InitMemoryHeader, _f.header_file()))
    }
}

pub(crate) struct ClearActivation;

impl FunctionDefinition for ClearActivation {
    fn name(&self, _f: &CFormatter) -> String {
        "clear_activation".into()
    }

    fn body(self, f: &CFormatter) -> String {
        f.streams()
            .filter_map(|s| f.static_buffer(s))
            .map(|b| b.reset_fresh(f))
            .join("\n")
    }

    fn arguments(&self, f: &CFormatter) -> Vec<Argument> {
        vec![MemoryStruct.into_argument(f).reference()]
    }

    fn key(&self) -> RequirementKey {
        RequirementKey::ClearActivation
    }

    fn file(&self, f: &CFormatter) -> std::path::PathBuf {
        f.monitor_file()
    }
}
