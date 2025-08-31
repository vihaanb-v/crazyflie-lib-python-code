use core::fmt::Display;
#[derive(Debug, Clone)]
			pub enum MonitorError {
            InstanceNotFound { stream: &'static str, instance: String },
			OutOfBoundsAccess { accessed_offset: usize, buffer_size: usize },
		}
pub struct Monitor {
streammemory: StreamMemory,
queue: Queue,
time: core::time::Duration,
spawned: Vec<Deadline>,
closed: Vec<StreamReference>,
}
impl Monitor {
pub fn new(start_time: core::time::Duration) -> Self {
Self {
            streammemory: StreamMemory::new(start_time),
queue: Queue::new(start_time),
time: start_time,
spawned: Vec::new(),
closed: Vec::new()
	}
}
}
struct StreamMemory {
state_x: StreamBuffer<f64, 1>,
state_y: StreamBuffer<f64, 1>,
state_z: StreamBuffer<f64, 1>,
multi_x: StreamBuffer<f64, 1>,
multi_y: StreamBuffer<f64, 1>,
multi_z: StreamBuffer<f64, 1>,
dx: StreamBuffer<f64, 1>,
dy: StreamBuffer<f64, 1>,
dz: StreamBuffer<f64, 1>,
net_drift: StreamBuffer<f64, 1>,
warning_minimal: StreamBuffer<bool, 1>,
trigger_0: StreamBuffer<String, 1>,
warning_excessive: StreamBuffer<bool, 1>,
trigger_1: StreamBuffer<String, 1>,
}
impl StreamMemory {
fn new(start_time: core::time::Duration) -> Self {
Self {
state_x: StreamBuffer::new(),
state_y: StreamBuffer::new(),
state_z: StreamBuffer::new(),
multi_x: StreamBuffer::new(),
multi_y: StreamBuffer::new(),
multi_z: StreamBuffer::new(),
dx: StreamBuffer::new(),
dy: StreamBuffer::new(),
dz: StreamBuffer::new(),
net_drift: StreamBuffer::new(),
warning_minimal: StreamBuffer::new(),
trigger_0: StreamBuffer::new(),
warning_excessive: StreamBuffer::new(),
trigger_1: StreamBuffer::new()
}
}
}
pub(crate) trait StreamBufferTrait<StreamType, const STREAM_SIZE: usize> {
    fn stream_buffer_as_mut(&mut self) -> &mut StreamBuffer<StreamType, STREAM_SIZE>;
    fn stream_buffer(&self) -> &StreamBuffer<StreamType, STREAM_SIZE>;

    fn get(&self, offset: usize) -> Result<Option<&StreamType>, MonitorError> {
        let stream_buffer = self.stream_buffer();
        let index = (stream_buffer.current + STREAM_SIZE - offset) % STREAM_SIZE;
        stream_buffer
            .values
            .get(index)
            .map(Option::as_ref)
            .ok_or_else(|| MonitorError::OutOfBoundsAccess {
                accessed_offset: offset,
                buffer_size: STREAM_SIZE,
            })
    }

    fn update(&mut self, new_value: StreamType) -> Result<(), MonitorError> {
        let stream_buffer = self.stream_buffer_as_mut();
        let current_index: usize = stream_buffer.current;
        let value = stream_buffer.values.get_mut(current_index).ok_or_else(|| {
            MonitorError::OutOfBoundsAccess {
                accessed_offset: current_index,
                buffer_size: STREAM_SIZE,
            }
        })?;
        *value = Some(new_value);
        stream_buffer.fresh = true;
        Ok(())
    }

    fn shift(&mut self) {
        let stream_buffer = self.stream_buffer_as_mut();
        stream_buffer.current = (stream_buffer.current + 1) % STREAM_SIZE;
    }

    fn is_fresh(&self) -> bool {
        let stream_buffer = self.stream_buffer();
        stream_buffer.fresh
    }

    fn clear_activation(&mut self) {
        let stream_buffer = self.stream_buffer_as_mut();
        stream_buffer.fresh = false;
    }
}

#[derive(Debug, Clone)]
pub(crate) struct StreamBuffer<StreamType, const STREAM_SIZE: usize> {
    values: [Option<StreamType>; STREAM_SIZE],
    current: usize,
    fresh: bool,
}

impl<StreamType, const STREAM_SIZE: usize> StreamBuffer<StreamType, STREAM_SIZE> {
    pub(crate) fn new() -> Self {
        let values = core::array::from_fn(|_| None);
        Self {
            values,
            current: 0,
            fresh: false,
        }
    }
}

impl<StreamType, const STREAM_SIZE: usize> StreamBufferTrait<StreamType, STREAM_SIZE>
    for StreamBuffer<StreamType, STREAM_SIZE>
{
    fn stream_buffer(&self) -> &StreamBuffer<StreamType, STREAM_SIZE> {
        self
    }

    fn stream_buffer_as_mut(&mut self) -> &mut StreamBuffer<StreamType, STREAM_SIZE> {
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum StreamReference {
State_x,
State_y,
State_z,
Multi_x,
Multi_y,
Multi_z,
Dx,
Dy,
Dz,
Net_drift,
Warning_minimal,
Trigger_0,
Warning_excessive,
Trigger_1,
}
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
enum Deadline {
Static100,
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
time: core::time::Duration,
deadline: Deadline,
}
impl State {
fn new_after(deadline: Deadline, time: core::time::Duration) -> Self {
match deadline {
            Deadline::Static100 => State {
        time: time + core::time::Duration::new(0, 100000000),
        deadline
    }
        }
}
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        other.time.cmp(&self.time)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
struct Queue {
heap: std::collections::BinaryHeap<State>,
}
impl Queue {
fn new(start_time: core::time::Duration) -> Self {
Self{heap: std::collections::BinaryHeap::from(vec![
State {

            time: start_time + core::time::Duration::new(0, 100000000),
            deadline: Deadline::Static100
        
}
])}
}
}
impl Queue {
fn pop(&mut self) -> Option<State> {
self.heap.pop()
}
}
impl Queue {
fn push(&mut self,state: State) -> Result<(), MonitorError> {
Ok(self.heap.push(state))
}
}
impl Queue {
fn collect_and_add(&mut self,mut spawned_streams: Vec<Deadline>, time: core::time::Duration) -> Result<(), MonitorError> {
spawned_streams.sort_unstable();
let deadlines = spawned_streams.into_iter().fold(Vec::new(), |mut acc, deadline| {
if let Some(last) = acc.last_mut() {
match (last, deadline) {
(_, _) => {}
};
} else {
acc.push(deadline);
}
acc
});
self.heap.extend(deadlines.into_iter().map(|deadline| State::new_after(deadline, time)));
Ok(())
}
}
impl Queue {
fn remove(&mut self,closed_streams: Vec<StreamReference>) -> () {
if !closed_streams.is_empty() {
self.heap = self
.heap
.drain()
.filter_map(|State { time, deadline }| {
match deadline {
_ => Some(State{time, deadline})
}
}).collect()
}
}
}
impl Queue {
fn next(&mut self,end: core::time::Duration, inclusive: bool) -> Result<Option<InternalEvent>, MonitorError> {
let mut current: Option<InternalEvent> = None;
while let Some(state) = self.pop() {
if (!inclusive && state.time >= end) || state.time > end {
self.push(state)?;
return Ok(current);
}
if let Some(current_event) = &current {
if state.time > current_event.time {
self.push(state)?;
return Ok(current);
}
}
let State { time, deadline } = state;
let current_event = current.get_or_insert_with(|| InternalEvent::empty(state.time));
match &deadline {
Deadline::Static100 => {current_event.static100 = true;}
}
let new_state = State::new_after(deadline, time);
self.push(new_state)?;
}
Ok(current)
}
}
impl Monitor {
fn clear(&mut self) -> Result<(), MonitorError> {
let spawned: Vec<_> = core::mem::take(self.spawned.as_mut());
let closed: Vec<StreamReference> = core::mem::take(self.closed.as_mut());
for &sr in closed.iter() {
match sr {
_ => unreachable!()
}
}
self.queue.collect_and_add(spawned, self.time)?;
self.queue.remove(closed);
Ok(())
}
}
pub struct Event {
multi_z: Option<f64>,
state_x: Option<f64>,
state_z: Option<f64>,
multi_y: Option<f64>,
state_y: Option<f64>,
multi_x: Option<f64>,
}
struct InternalEvent {
state_x: Option<f64>,
state_y: Option<f64>,
state_z: Option<f64>,
multi_x: Option<f64>,
multi_y: Option<f64>,
multi_z: Option<f64>,
static100: bool,
time: core::time::Duration,
}
impl InternalEvent {
fn from_event(time: core::time::Duration, event: Event) -> InternalEvent {
Self {
state_x: event.state_x
,state_y: event.state_y
,state_z: event.state_z
,multi_x: event.multi_x
,multi_y: event.multi_y
,multi_z: event.multi_z
,static100: false
,time: time
}
}
}
impl InternalEvent {
fn empty(time: core::time::Duration) -> Self {
Self { state_x: None,
state_y: None,
state_z: None,
multi_x: None,
multi_y: None,
multi_z: None,
static100: false,
time: time }
}
}
pub struct Verdict {
dx: Option<f64>,
dy: Option<f64>,
dz: Option<f64>,
net_drift: Option<f64>,
warning_minimal: Option<bool>,
trigger_0: Option<String>,
warning_excessive: Option<bool>,
trigger_1: Option<String>,
time: core::time::Duration,
}
impl Verdict {
fn new(monitor: &mut Monitor) -> Result<Verdict, MonitorError> {
Ok(Self {
dx: monitor.dx_get()?,
dy: monitor.dy_get()?,
dz: monitor.dz_get()?,
net_drift: monitor.net_drift_get()?,
warning_minimal: monitor.warning_minimal_get()?,
trigger_0: monitor.trigger_0_get()?,
warning_excessive: monitor.warning_excessive_get()?,
trigger_1: monitor.trigger_1_get()?,
time: monitor.time
})
}
}

impl Display for Verdict {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{},", self.dx.as_ref().map(|v|v.to_string()).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.dy.as_ref().map(|v|v.to_string()).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.dz.as_ref().map(|v|v.to_string()).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.net_drift.as_ref().map(|v|v.to_string()).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.warning_minimal.as_ref().map(|v|v.to_string()).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.trigger_0.as_ref().map(|v|format!("\"{v}\"")).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.warning_excessive.as_ref().map(|v|v.to_string()).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.trigger_1.as_ref().map(|v|format!("\"{v}\"")).unwrap_or_else(||"#".into()))?;
		writeln!(f, "{}", self.time.as_secs_f64())
        }
    }
impl Monitor {
fn expr_0(&mut self) -> Result<bool, MonitorError> {
Ok(self.warning_minimal()?)
}
}
impl Monitor {
fn expr_1(&mut self) -> Result<bool, MonitorError> {
Ok(self.warning_excessive()?)
}
}
impl Monitor {
fn clear_activations(&mut self) -> () {
self.streammemory.state_x.clear_activation();
self.streammemory.state_y.clear_activation();
self.streammemory.state_z.clear_activation();
self.streammemory.multi_x.clear_activation();
self.streammemory.multi_y.clear_activation();
self.streammemory.multi_z.clear_activation();
self.streammemory.dx.clear_activation();
self.streammemory.dy.clear_activation();
self.streammemory.dz.clear_activation();
self.streammemory.net_drift.clear_activation();
self.streammemory.warning_minimal.clear_activation();
self.streammemory.trigger_0.clear_activation();
self.streammemory.warning_excessive.clear_activation();
self.streammemory.trigger_1.clear_activation();
}
}
impl Monitor {
fn cycle(&mut self,internalevent: InternalEvent) -> Result<Verdict, MonitorError> {
self.time = internalevent.time;
if internalevent.state_x.is_some() {
self.streammemory.state_x.shift();
self.eval_state_x(internalevent.state_x.expect("Checked the existence with the guard"))?;
}
if internalevent.state_y.is_some() {
self.streammemory.state_y.shift();
self.eval_state_y(internalevent.state_y.expect("Checked the existence with the guard"))?;
}
if internalevent.state_z.is_some() {
self.streammemory.state_z.shift();
self.eval_state_z(internalevent.state_z.expect("Checked the existence with the guard"))?;
}
if internalevent.multi_x.is_some() {
self.streammemory.multi_x.shift();
self.eval_multi_x(internalevent.multi_x.expect("Checked the existence with the guard"))?;
}
if internalevent.multi_y.is_some() {
self.streammemory.multi_y.shift();
self.eval_multi_y(internalevent.multi_y.expect("Checked the existence with the guard"))?;
}
if internalevent.multi_z.is_some() {
self.streammemory.multi_z.shift();
self.eval_multi_z(internalevent.multi_z.expect("Checked the existence with the guard"))?;
}
if (internalevent.state_x.is_some() && internalevent.multi_x.is_some()) {
self.streammemory.dx.shift();
}
if (internalevent.state_y.is_some() && internalevent.multi_y.is_some()) {
self.streammemory.dy.shift();
}
if (internalevent.state_z.is_some() && internalevent.multi_z.is_some()) {
self.streammemory.dz.shift();
}
if (((((internalevent.state_x.is_some() && internalevent.state_y.is_some()) && internalevent.state_z.is_some()) && internalevent.multi_x.is_some()) && internalevent.multi_y.is_some()) && internalevent.multi_z.is_some()) {
self.streammemory.net_drift.shift();
}
if internalevent.static100 {
self.streammemory.warning_minimal.shift();
}
if internalevent.static100 {
self.streammemory.warning_excessive.shift();
}
if (internalevent.state_x.is_some() && internalevent.multi_x.is_some()) {
self.eval_dx_0()?;
}
if (internalevent.state_y.is_some() && internalevent.multi_y.is_some()) {
self.eval_dy_0()?;
}
if (internalevent.state_z.is_some() && internalevent.multi_z.is_some()) {
self.eval_dz_0()?;
}
if internalevent.static100 {
self.eval_warning_minimal_0()?;
}
if internalevent.static100 {
self.eval_warning_excessive_0()?;
}
if (((((internalevent.state_x.is_some() && internalevent.state_y.is_some()) && internalevent.state_z.is_some()) && internalevent.multi_x.is_some()) && internalevent.multi_y.is_some()) && internalevent.multi_z.is_some()) {
self.eval_net_drift_0()?;
}
if (internalevent.static100 && self.expr_0()?) {
self.streammemory.trigger_0.shift();
}
if (internalevent.static100 && self.expr_1()?) {
self.streammemory.trigger_1.shift();
}
if (internalevent.static100 && self.expr_0()?) {
self.eval_trigger_0_0()?;
}
if (internalevent.static100 && self.expr_1()?) {
self.eval_trigger_1_0()?;
}
let verdict = Verdict::new(self)?;
self.clear()?;
self.clear_activations();
Ok(verdict)
}
}
impl Monitor {
pub(crate) fn state_x(&self) -> Result<f64, MonitorError> {
Ok(self.streammemory.state_x.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn state_y(&self) -> Result<f64, MonitorError> {
Ok(self.streammemory.state_y.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn state_z(&self) -> Result<f64, MonitorError> {
Ok(self.streammemory.state_z.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn multi_x(&self) -> Result<f64, MonitorError> {
Ok(self.streammemory.multi_x.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn multi_y(&self) -> Result<f64, MonitorError> {
Ok(self.streammemory.multi_y.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn multi_z(&self) -> Result<f64, MonitorError> {
Ok(self.streammemory.multi_z.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn dx(&self) -> Result<f64, MonitorError> {
Ok(self.streammemory.dx.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn dy(&self) -> Result<f64, MonitorError> {
Ok(self.streammemory.dy.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn dz(&self) -> Result<f64, MonitorError> {
Ok(self.streammemory.dz.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn warning_minimal(&self) -> Result<bool, MonitorError> {
Ok(self.streammemory.warning_minimal.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn warning_excessive(&self) -> Result<bool, MonitorError> {
Ok(self.streammemory.warning_excessive.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn net_drift_hold(&self) -> Result<Option<f64>, MonitorError> {
Ok(self.streammemory.net_drift.get(0)?.cloned())
}
}
impl Monitor {
pub(crate) fn dx_get(&self) -> Result<Option<f64>, MonitorError> {
Ok(if self.streammemory.dx.is_fresh() {
self.streammemory.dx.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn dy_get(&self) -> Result<Option<f64>, MonitorError> {
Ok(if self.streammemory.dy.is_fresh() {
self.streammemory.dy.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn dz_get(&self) -> Result<Option<f64>, MonitorError> {
Ok(if self.streammemory.dz.is_fresh() {
self.streammemory.dz.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn net_drift_get(&self) -> Result<Option<f64>, MonitorError> {
Ok(if self.streammemory.net_drift.is_fresh() {
self.streammemory.net_drift.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn warning_minimal_get(&self) -> Result<Option<bool>, MonitorError> {
Ok(if self.streammemory.warning_minimal.is_fresh() {
self.streammemory.warning_minimal.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn trigger_0_get(&self) -> Result<Option<String>, MonitorError> {
Ok(if self.streammemory.trigger_0.is_fresh() {
self.streammemory.trigger_0.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn warning_excessive_get(&self) -> Result<Option<bool>, MonitorError> {
Ok(if self.streammemory.warning_excessive.is_fresh() {
self.streammemory.warning_excessive.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn trigger_1_get(&self) -> Result<Option<String>, MonitorError> {
Ok(if self.streammemory.trigger_1.is_fresh() {
self.streammemory.trigger_1.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn eval_state_x(&mut self,new_value: f64) -> Result<(), MonitorError> {

self.streammemory.state_x.update(new_value)
}
}
impl Monitor {
pub(crate) fn eval_state_y(&mut self,new_value: f64) -> Result<(), MonitorError> {

self.streammemory.state_y.update(new_value)
}
}
impl Monitor {
pub(crate) fn eval_state_z(&mut self,new_value: f64) -> Result<(), MonitorError> {

self.streammemory.state_z.update(new_value)
}
}
impl Monitor {
pub(crate) fn eval_multi_x(&mut self,new_value: f64) -> Result<(), MonitorError> {

self.streammemory.multi_x.update(new_value)
}
}
impl Monitor {
pub(crate) fn eval_multi_y(&mut self,new_value: f64) -> Result<(), MonitorError> {

self.streammemory.multi_y.update(new_value)
}
}
impl Monitor {
pub(crate) fn eval_multi_z(&mut self,new_value: f64) -> Result<(), MonitorError> {

self.streammemory.multi_z.update(new_value)
}
}
impl Monitor {
pub(crate) fn eval_dx_0(&mut self) -> Result<(), MonitorError> {
let new_value = (self.state_x()? - self.multi_x()?);
;
self.streammemory.dx.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_dy_0(&mut self) -> Result<(), MonitorError> {
let new_value = (self.state_y()? - self.multi_y()?);
;
self.streammemory.dy.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_dz_0(&mut self) -> Result<(), MonitorError> {
let new_value = (self.state_z()? - self.multi_z()?);
;
self.streammemory.dz.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_net_drift_0(&mut self) -> Result<(), MonitorError> {
let new_value = ((((self.dx()? * self.dx()?) + (self.dy()? * self.dy()?)) + (self.dz()? * self.dz()?))).sqrt();
;
self.streammemory.net_drift.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_warning_minimal_0(&mut self) -> Result<(), MonitorError> {
let new_value = (self.net_drift_hold()?.unwrap_or(0f64) > 0.25f64);
;
self.streammemory.warning_minimal.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_trigger_0_0(&mut self) -> Result<(), MonitorError> {
let new_value = "Warning: Difference in position estimate between stateEstimate and MultiRanger estimate > 0.35 m".to_owned();
;
self.streammemory.trigger_0.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_warning_excessive_0(&mut self) -> Result<(), MonitorError> {
let new_value = (self.net_drift_hold()?.unwrap_or(0f64) > 0.5f64);
;
self.streammemory.warning_excessive.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_trigger_1_0(&mut self) -> Result<(), MonitorError> {
let new_value = "Warning: Difference in position estimate between stateEstimate and MultiRanger estimate > 0.35 m".to_owned();
;
self.streammemory.trigger_1.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub fn accept_event(&mut self,event: Event, time: core::time::Duration) -> Result<Vec<Verdict>, MonitorError> {
let event = InternalEvent::from_event(time, event);
let mut verdicts = self.accept_time(time)?;
verdicts.push(self.cycle(event)?);
Ok(verdicts)
}
}
impl Monitor {
pub fn accept_time(&mut self,time: core::time::Duration) -> Result<Vec<Verdict>, MonitorError> {
let mut verdicts = Vec::new();
while let Some(timed_event) = self.queue.next(time, false)? {
let verdict = self.cycle(timed_event)?;
verdicts.push(verdict)
}
return Ok(verdicts);
}
}
impl Monitor {
pub fn close(&mut self,time: core::time::Duration) -> Result<Vec<Verdict>, MonitorError> {
let mut verdicts = Vec::new();
while let Some(timed_event) = self.queue.next(time, true)? {
let verdict = self.cycle(timed_event)?;
verdicts.push(verdict)
}
return Ok(verdicts);
}
}