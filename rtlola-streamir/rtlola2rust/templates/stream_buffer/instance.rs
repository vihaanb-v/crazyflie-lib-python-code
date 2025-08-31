#[derive(Debug, Clone)]
pub(crate) struct InstanceStreamBuffer<
    Parameter: Eq + core::hash::Hash + Clone,
    StreamType: Clone,
    const STREAM_SIZE: usize,
    {%- if heapless %}
    const NUM_INSTANCES: usize
    {%- endif %}
> {
    {%- if heapless %}
    stream_buffer: heapless::FnvIndexMap<Parameter, StreamBuffer<StreamType, STREAM_SIZE>, NUM_INSTANCES>,
    {%- else %}
    stream_buffer: std::collections::HashMap<Parameter, StreamBuffer<StreamType, STREAM_SIZE>>,
    {%- endif %}
}

{%- if heapless %}
impl<Parameter: Eq + core::hash::Hash + Clone, StreamType: Clone, const STREAM_SIZE: usize, const NUM_INSTANCES: usize>
    InstanceStreamBuffer<Parameter, StreamType, STREAM_SIZE, NUM_INSTANCES>
{%- else %}
impl<Parameter: Eq + core::hash::Hash + Clone, StreamType: Clone, const STREAM_SIZE: usize>
    InstanceStreamBuffer<Parameter, StreamType, STREAM_SIZE>
{%- endif %}
{
    pub(crate) fn new() -> Self {
        InstanceStreamBuffer {
            {%- if heapless %}
            stream_buffer: heapless::FnvIndexMap::new(),
            {%- else %}
            stream_buffer: std::collections::HashMap::new(),
            {%- endif %}
        }
    }

    pub(crate) fn get_instance(
        &self,
        parameter: &Parameter,
    ) -> Option<&StreamBuffer<StreamType, STREAM_SIZE>> {
        self.stream_buffer.get(parameter)
    }

    pub(crate) fn get_instance_mut(
        &mut self,
        parameter: &Parameter,
    ) -> Option<&mut StreamBuffer<StreamType, STREAM_SIZE>> {
        self.stream_buffer.get_mut(parameter)
    }

    pub(crate) fn is_alive(&self, parameter: &Parameter) -> bool {
        self.stream_buffer.contains_key(parameter)
    }

    pub(crate) fn spawn(&mut self, parameter: Parameter) -> Result<(), MonitorError> {
        {%- if heapless %}
        if let heapless::Entry::Vacant(v) = self
            .stream_buffer
            .entry(parameter) {
                v.insert(StreamBuffer::new()).map_err(|_| MonitorError::TooManyInstances)?;
            }
        {%- else %}
        let _ = self
            .stream_buffer
            .entry(parameter)
            .or_insert_with(StreamBuffer::new);
        {%- endif %}
        Ok(())
    }

    pub(crate) fn close(&mut self, parameter: Parameter) -> Result<(), MonitorError> {
        let res = self.stream_buffer.remove(&parameter);
        assert!(res.is_some());
        Ok(())
    }

    {%- if heapless %}
    pub(crate) fn alive_parameters(&self) -> heapless::Vec<Parameter, NUM_INSTANCES> {
    {%- else %}
    pub(crate) fn alive_parameters(&self) -> Vec<Parameter> {
    {%- endif %}
        self.stream_buffer.keys().cloned().collect()
    }

    pub(crate) fn clear_activation(&mut self) {
        for instance in self.stream_buffer.values_mut() {
            instance.clear_activation();
        }
    }

    {%- if heapless %}
    fn fresh_instances(&self) -> Result<heapless::FnvIndexMap<Parameter, StreamType, NUM_INSTANCES>, MonitorError> {
    {%- else %}
    fn fresh_instances(&self) -> Result<std::collections::HashMap<Parameter, StreamType>, MonitorError> {
    {%- endif %}
        self.stream_buffer
            .iter()
            .filter(|(_, s)| s.fresh)
            .map(|(p, s)| Ok((p.clone(), s.get(0)?.cloned().unwrap())))
            .collect()
    }
}
