use core::fmt::Display;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::Add;
#[derive(Debug, Clone)]
			pub enum MonitorError {
            InstanceNotFound { stream: &'static str, instance: String },
			OutOfBoundsAccess { accessed_offset: usize, buffer_size: usize },
		}
pub struct Monitor {
streammemory: StreamMemory,
queue: Queue,
windowsmemory: WindowsMemory,
time: core::time::Duration,
spawned: Vec<Deadline>,
closed: Vec<StreamReference>,
}
impl Monitor {
pub fn new(start_time: core::time::Duration) -> Self {
Self {
            streammemory: StreamMemory::new(start_time),
queue: Queue::new(start_time),
windowsmemory: WindowsMemory::new(start_time),
time: start_time,
spawned: Vec::new(),
closed: Vec::new()
	}
}
}
struct StreamMemory {
x: StreamBuffer<f64, 1>,
y: StreamBuffer<f64, 1>,
z: StreamBuffer<f64, 1>,
p0_x: StreamBuffer<f64, 1>,
p0_y: StreamBuffer<f64, 1>,
p0_z: StreamBuffer<f64, 1>,
p1_x: StreamBuffer<f64, 1>,
p1_y: StreamBuffer<f64, 1>,
p1_z: StreamBuffer<f64, 1>,
dx: StreamBuffer<f64, 1>,
dy: StreamBuffer<f64, 1>,
dz: StreamBuffer<f64, 1>,
len2: StreamBuffer<f64, 1>,
seg_valid: StreamBuffer<bool, 1>,
vx: StreamBuffer<f64, 1>,
vy: StreamBuffer<f64, 1>,
vz: StreamBuffer<f64, 1>,
t_raw: StreamBuffer<f64, 1>,
t: StreamBuffer<f64, 1>,
c_x: StreamBuffer<f64, 1>,
c_y: StreamBuffer<f64, 1>,
c_z: StreamBuffer<f64, 1>,
cross_track_drift: StreamBuffer<f64, 1>,
warning: StreamBuffer<bool, 1>,
trigger_0: StreamBuffer<String, 1>,
violation: StreamBuffer<bool, 1>,
trigger_1: StreamBuffer<String, 1>,
}
impl StreamMemory {
fn new(start_time: core::time::Duration) -> Self {
Self {
x: StreamBuffer::new(),
y: StreamBuffer::new(),
z: StreamBuffer::new(),
p0_x: StreamBuffer::new(),
p0_y: StreamBuffer::new(),
p0_z: StreamBuffer::new(),
p1_x: StreamBuffer::new(),
p1_y: StreamBuffer::new(),
p1_z: StreamBuffer::new(),
dx: StreamBuffer::new(),
dy: StreamBuffer::new(),
dz: StreamBuffer::new(),
len2: StreamBuffer::new(),
seg_valid: StreamBuffer::new(),
vx: StreamBuffer::new(),
vy: StreamBuffer::new(),
vz: StreamBuffer::new(),
t_raw: StreamBuffer::new(),
t: StreamBuffer::new(),
c_x: StreamBuffer::new(),
c_y: StreamBuffer::new(),
c_z: StreamBuffer::new(),
cross_track_drift: StreamBuffer::new(),
warning: StreamBuffer::new(),
trigger_0: StreamBuffer::new(),
violation: StreamBuffer::new(),
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
X,
Y,
Z,
P0_x,
P0_y,
P0_z,
P1_x,
P1_y,
P1_z,
Dx,
Dy,
Dz,
Len2,
Seg_valid,
Vx,
Vy,
Vz,
T_raw,
T,
C_x,
C_y,
C_z,
Cross_track_drift,
Warning,
Trigger_0,
Violation,
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
struct WindowsMemory {
sliding0: SlidingWindowBuffer<ConjunctionAggregation, 25, false>,
}
impl WindowsMemory {
fn new(start_time: core::time::Duration) -> Self {
Self {
sliding0: SlidingWindowBuffer::new(start_time, core::time::Duration::new(0, 100000000))
}
}
}
trait WindowAggregation {
    type BucketType: Copy;
    type ValueType: Copy;
    type ResultType: Copy;

    fn aggregate(buffer1: Self::BucketType, buffer2: Self::BucketType) -> Self::BucketType;
    fn map(value: Self::ValueType, time: Duration) -> Self::BucketType;
    fn lower(buffer: Self::BucketType) -> Self::ResultType;
    fn initial_value(time: Duration) -> Self::BucketType;
}

trait Window<AggregationFunction: WindowAggregation, const NUM_BUCKETS: usize, const WAIT: bool> {
    fn shift(&mut self, index: usize, time: Duration);
    fn accept_value(&mut self, _time: Duration, new_value: AggregationFunction::ValueType);
    fn buckets(&self) -> &[AggregationFunction::BucketType; NUM_BUCKETS];
    fn current_bucket(&self) -> usize;
    fn index(&self, ts: Duration) -> i32;
    fn start_time(&self) -> Duration;
    fn bucket_duration(&self) -> Duration;

    fn get_value(&mut self, time: Duration) -> Option<AggregationFunction::ResultType> {
        if WAIT && time < self.start_time() + (self.bucket_duration() * NUM_BUCKETS as u32) {
            return None;
        }

        let index = self.index(time) as usize;
        if index >= NUM_BUCKETS {
            Some(AggregationFunction::lower(
                AggregationFunction::initial_value(time),
            ))
        } else {
            Some(AggregationFunction::lower(
                self.buckets()
                    .iter()
                    .cycle()
                    .skip(self.current_bucket())
                    .take(NUM_BUCKETS - index)
                    .cloned()
                    .reduce(|b1, b2| AggregationFunction::aggregate(b2, b1))
                    .unwrap(),
            ))
        }
    }
}

#[derive(Debug, Clone)]
struct ConjunctionAggregation;
impl WindowAggregation for ConjunctionAggregation {
    type BucketType = bool;
    type ResultType = bool;
    type ValueType = bool;

    fn aggregate(current: Self::BucketType, value: Self::BucketType) -> Self::BucketType {
        current && value
    }

    fn map(value: Self::ValueType, time: Duration) -> Self::BucketType {
        value
    }

    fn lower(value: Self::BucketType) -> Self::ResultType {
        value
    }

    fn initial_value(_time: Duration) -> Self::BucketType {
        true
    }
}

#[derive(Debug, Clone)]
struct SlidingWindowBuffer<
    AggregationFunction: WindowAggregation,
    const NUM_BUCKETS: usize,
    const WAIT: bool,
> {
    buckets: [AggregationFunction::BucketType; NUM_BUCKETS],
    current: usize,
    current_bucket_end: Duration,
    bucket_size: Duration,
    start_time: Duration,
    _phantom: PhantomData<AggregationFunction>,
}

impl<AggregationFunction: WindowAggregation, const NUM_BUCKETS: usize, const WAIT: bool>
    SlidingWindowBuffer<AggregationFunction, NUM_BUCKETS, WAIT>
{
    fn new(start_time: Duration, bucket_size: Duration) -> Self {
        let buckets = core::array::from_fn(|_| AggregationFunction::initial_value(start_time));

        SlidingWindowBuffer {
            buckets,
            current: 0,
            current_bucket_end: start_time + bucket_size,
            _phantom: PhantomData,
            bucket_size,
            start_time,
        }
    }
}

impl<AggregationFunction: WindowAggregation, const NUM_BUCKETS: usize, const WAIT: bool>
    Window<AggregationFunction, NUM_BUCKETS, WAIT>
    for SlidingWindowBuffer<AggregationFunction, NUM_BUCKETS, WAIT>
{
    fn index(&self, ts: Duration) -> i32 {
        let diff = ts - (self.current_bucket_end - self.bucket_size);
        let index = (diff.as_nanos() / self.bucket_size.as_nanos()) as i32;
        if diff == self.bucket_size * (index as u32) {
            index - 1
        } else {
            index
        }
    }

    fn shift(&mut self, index: usize, time: Duration) {
        self.current_bucket_end += self.bucket_size * (index as u32);
        if index >= NUM_BUCKETS {
            self.buckets
                .iter_mut()
                .for_each(|bucket| *bucket = AggregationFunction::initial_value(time));
            return;
        }
        self.current = (self.current + NUM_BUCKETS - index) % NUM_BUCKETS;
        let (second_half, first_half) = self.buckets.split_at_mut(self.current);
        first_half
            .iter_mut()
            .chain(second_half.iter_mut())
            .take(index)
            .for_each(|bucket| *bucket = AggregationFunction::initial_value(time));
    }

    fn accept_value(&mut self, time: Duration, new_value: AggregationFunction::ValueType) {
        let index = self.index(time);
        if index == -1 {
            debug_assert_eq!(self.current, 0);
            self.buckets
                .get_mut(1)
                .map(|v| *v = AggregationFunction::map(new_value, time));
            return;
        }
        self.shift(index as usize, time);
        let cur_bucket = self.buckets.get_mut(self.current).expect("Cannot fail");
        let new_value = AggregationFunction::map(new_value, time);

        *cur_bucket = AggregationFunction::aggregate(*cur_bucket, new_value)
    }

    fn buckets(&self) -> &[AggregationFunction::BucketType; NUM_BUCKETS] {
        &self.buckets
    }

    fn current_bucket(&self) -> usize {
        self.current
    }

    fn start_time(&self) -> Duration {
        self.start_time
    }
    fn bucket_duration(&self) -> Duration {
        self.bucket_size
    }
}

pub struct Event {
p1_y: Option<f64>,
p1_z: Option<f64>,
p1_x: Option<f64>,
x: Option<f64>,
y: Option<f64>,
p0_z: Option<f64>,
p0_y: Option<f64>,
z: Option<f64>,
p0_x: Option<f64>,
}
struct InternalEvent {
x: Option<f64>,
y: Option<f64>,
z: Option<f64>,
p0_x: Option<f64>,
p0_y: Option<f64>,
p0_z: Option<f64>,
p1_x: Option<f64>,
p1_y: Option<f64>,
p1_z: Option<f64>,
static100: bool,
time: core::time::Duration,
}
impl InternalEvent {
fn from_event(time: core::time::Duration, event: Event) -> InternalEvent {
Self {
x: event.x
,y: event.y
,z: event.z
,p0_x: event.p0_x
,p0_y: event.p0_y
,p0_z: event.p0_z
,p1_x: event.p1_x
,p1_y: event.p1_y
,p1_z: event.p1_z
,static100: false
,time: time
}
}
}
impl InternalEvent {
fn empty(time: core::time::Duration) -> Self {
Self { x: None,
y: None,
z: None,
p0_x: None,
p0_y: None,
p0_z: None,
p1_x: None,
p1_y: None,
p1_z: None,
static100: false,
time: time }
}
}
pub struct Verdict {
dx: Option<f64>,
dy: Option<f64>,
dz: Option<f64>,
len2: Option<f64>,
seg_valid: Option<bool>,
vx: Option<f64>,
vy: Option<f64>,
vz: Option<f64>,
t_raw: Option<f64>,
t: Option<f64>,
c_x: Option<f64>,
c_y: Option<f64>,
c_z: Option<f64>,
cross_track_drift: Option<f64>,
warning: Option<bool>,
trigger_0: Option<String>,
violation: Option<bool>,
trigger_1: Option<String>,
time: core::time::Duration,
}
impl Verdict {
fn new(monitor: &mut Monitor) -> Result<Verdict, MonitorError> {
Ok(Self {
dx: monitor.dx_get()?,
dy: monitor.dy_get()?,
dz: monitor.dz_get()?,
len2: monitor.len2_get()?,
seg_valid: monitor.seg_valid_get()?,
vx: monitor.vx_get()?,
vy: monitor.vy_get()?,
vz: monitor.vz_get()?,
t_raw: monitor.t_raw_get()?,
t: monitor.t_get()?,
c_x: monitor.c_x_get()?,
c_y: monitor.c_y_get()?,
c_z: monitor.c_z_get()?,
cross_track_drift: monitor.cross_track_drift_get()?,
warning: monitor.warning_get()?,
trigger_0: monitor.trigger_0_get()?,
violation: monitor.violation_get()?,
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
write!(f, "{},", self.len2.as_ref().map(|v|v.to_string()).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.seg_valid.as_ref().map(|v|v.to_string()).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.vx.as_ref().map(|v|v.to_string()).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.vy.as_ref().map(|v|v.to_string()).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.vz.as_ref().map(|v|v.to_string()).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.t_raw.as_ref().map(|v|v.to_string()).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.t.as_ref().map(|v|v.to_string()).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.c_x.as_ref().map(|v|v.to_string()).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.c_y.as_ref().map(|v|v.to_string()).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.c_z.as_ref().map(|v|v.to_string()).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.cross_track_drift.as_ref().map(|v|v.to_string()).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.warning.as_ref().map(|v|v.to_string()).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.trigger_0.as_ref().map(|v|format!("\"{v}\"")).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.violation.as_ref().map(|v|v.to_string()).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.trigger_1.as_ref().map(|v|format!("\"{v}\"")).unwrap_or_else(||"#".into()))?;
		writeln!(f, "{}", self.time.as_secs_f64())
        }
    }
impl Monitor {
fn expr_0(&mut self) -> Result<bool, MonitorError> {
Ok(self.warning()?)
}
}
impl Monitor {
fn expr_1(&mut self) -> Result<bool, MonitorError> {
Ok(self.violation()?)
}
}
impl Monitor {
fn clear_activations(&mut self) -> () {
self.streammemory.x.clear_activation();
self.streammemory.y.clear_activation();
self.streammemory.z.clear_activation();
self.streammemory.p0_x.clear_activation();
self.streammemory.p0_y.clear_activation();
self.streammemory.p0_z.clear_activation();
self.streammemory.p1_x.clear_activation();
self.streammemory.p1_y.clear_activation();
self.streammemory.p1_z.clear_activation();
self.streammemory.dx.clear_activation();
self.streammemory.dy.clear_activation();
self.streammemory.dz.clear_activation();
self.streammemory.len2.clear_activation();
self.streammemory.seg_valid.clear_activation();
self.streammemory.vx.clear_activation();
self.streammemory.vy.clear_activation();
self.streammemory.vz.clear_activation();
self.streammemory.t_raw.clear_activation();
self.streammemory.t.clear_activation();
self.streammemory.c_x.clear_activation();
self.streammemory.c_y.clear_activation();
self.streammemory.c_z.clear_activation();
self.streammemory.cross_track_drift.clear_activation();
self.streammemory.warning.clear_activation();
self.streammemory.trigger_0.clear_activation();
self.streammemory.violation.clear_activation();
self.streammemory.trigger_1.clear_activation();
}
}
impl Monitor {
fn cycle(&mut self,internalevent: InternalEvent) -> Result<Verdict, MonitorError> {
self.time = internalevent.time;
if internalevent.x.is_some() {
self.streammemory.x.shift();
self.eval_x(internalevent.x.expect("Checked the existence with the guard"))?;
}
if internalevent.y.is_some() {
self.streammemory.y.shift();
self.eval_y(internalevent.y.expect("Checked the existence with the guard"))?;
}
if internalevent.z.is_some() {
self.streammemory.z.shift();
self.eval_z(internalevent.z.expect("Checked the existence with the guard"))?;
}
if internalevent.p0_x.is_some() {
self.streammemory.p0_x.shift();
self.eval_p0_x(internalevent.p0_x.expect("Checked the existence with the guard"))?;
}
if internalevent.p0_y.is_some() {
self.streammemory.p0_y.shift();
self.eval_p0_y(internalevent.p0_y.expect("Checked the existence with the guard"))?;
}
if internalevent.p0_z.is_some() {
self.streammemory.p0_z.shift();
self.eval_p0_z(internalevent.p0_z.expect("Checked the existence with the guard"))?;
}
if internalevent.p1_x.is_some() {
self.streammemory.p1_x.shift();
self.eval_p1_x(internalevent.p1_x.expect("Checked the existence with the guard"))?;
}
if internalevent.p1_y.is_some() {
self.streammemory.p1_y.shift();
self.eval_p1_y(internalevent.p1_y.expect("Checked the existence with the guard"))?;
}
if internalevent.p1_z.is_some() {
self.streammemory.p1_z.shift();
self.eval_p1_z(internalevent.p1_z.expect("Checked the existence with the guard"))?;
}
if (internalevent.p0_x.is_some() && internalevent.p1_x.is_some()) {
self.streammemory.dx.shift();
}
if (internalevent.p0_y.is_some() && internalevent.p1_y.is_some()) {
self.streammemory.dy.shift();
}
if (internalevent.p0_z.is_some() && internalevent.p1_z.is_some()) {
self.streammemory.dz.shift();
}
if (((((internalevent.p0_x.is_some() && internalevent.p0_y.is_some()) && internalevent.p0_z.is_some()) && internalevent.p1_x.is_some()) && internalevent.p1_y.is_some()) && internalevent.p1_z.is_some()) {
self.streammemory.len2.shift();
}
if (((((internalevent.p0_x.is_some() && internalevent.p0_y.is_some()) && internalevent.p0_z.is_some()) && internalevent.p1_x.is_some()) && internalevent.p1_y.is_some()) && internalevent.p1_z.is_some()) {
self.streammemory.seg_valid.shift();
}
if (internalevent.x.is_some() && internalevent.p0_x.is_some()) {
self.streammemory.vx.shift();
}
if (internalevent.y.is_some() && internalevent.p0_y.is_some()) {
self.streammemory.vy.shift();
}
if (internalevent.z.is_some() && internalevent.p0_z.is_some()) {
self.streammemory.vz.shift();
}
if ((((((((internalevent.x.is_some() && internalevent.y.is_some()) && internalevent.z.is_some()) && internalevent.p0_x.is_some()) && internalevent.p0_y.is_some()) && internalevent.p0_z.is_some()) && internalevent.p1_x.is_some()) && internalevent.p1_y.is_some()) && internalevent.p1_z.is_some()) {
self.streammemory.t_raw.shift();
}
if ((((((((internalevent.x.is_some() && internalevent.y.is_some()) && internalevent.z.is_some()) && internalevent.p0_x.is_some()) && internalevent.p0_y.is_some()) && internalevent.p0_z.is_some()) && internalevent.p1_x.is_some()) && internalevent.p1_y.is_some()) && internalevent.p1_z.is_some()) {
self.streammemory.t.shift();
}
if ((((((((internalevent.x.is_some() && internalevent.y.is_some()) && internalevent.z.is_some()) && internalevent.p0_x.is_some()) && internalevent.p0_y.is_some()) && internalevent.p0_z.is_some()) && internalevent.p1_x.is_some()) && internalevent.p1_y.is_some()) && internalevent.p1_z.is_some()) {
self.streammemory.c_x.shift();
}
if ((((((((internalevent.x.is_some() && internalevent.y.is_some()) && internalevent.z.is_some()) && internalevent.p0_x.is_some()) && internalevent.p0_y.is_some()) && internalevent.p0_z.is_some()) && internalevent.p1_x.is_some()) && internalevent.p1_y.is_some()) && internalevent.p1_z.is_some()) {
self.streammemory.c_y.shift();
}
if ((((((((internalevent.x.is_some() && internalevent.y.is_some()) && internalevent.z.is_some()) && internalevent.p0_x.is_some()) && internalevent.p0_y.is_some()) && internalevent.p0_z.is_some()) && internalevent.p1_x.is_some()) && internalevent.p1_y.is_some()) && internalevent.p1_z.is_some()) {
self.streammemory.c_z.shift();
}
if ((((((((internalevent.x.is_some() && internalevent.y.is_some()) && internalevent.z.is_some()) && internalevent.p0_x.is_some()) && internalevent.p0_y.is_some()) && internalevent.p0_z.is_some()) && internalevent.p1_x.is_some()) && internalevent.p1_y.is_some()) && internalevent.p1_z.is_some()) {
self.streammemory.cross_track_drift.shift();
}
if internalevent.static100 {
self.streammemory.warning.shift();
}
if internalevent.static100 {
self.streammemory.violation.shift();
}
if (internalevent.p0_x.is_some() && internalevent.p1_x.is_some()) {
self.eval_dx_0()?;
}
if (internalevent.p0_y.is_some() && internalevent.p1_y.is_some()) {
self.eval_dy_0()?;
}
if (internalevent.p0_z.is_some() && internalevent.p1_z.is_some()) {
self.eval_dz_0()?;
}
if (internalevent.x.is_some() && internalevent.p0_x.is_some()) {
self.eval_vx_0()?;
}
if (internalevent.y.is_some() && internalevent.p0_y.is_some()) {
self.eval_vy_0()?;
}
if (internalevent.z.is_some() && internalevent.p0_z.is_some()) {
self.eval_vz_0()?;
}
if internalevent.static100 {
self.eval_warning_0()?;
}
if (((((internalevent.p0_x.is_some() && internalevent.p0_y.is_some()) && internalevent.p0_z.is_some()) && internalevent.p1_x.is_some()) && internalevent.p1_y.is_some()) && internalevent.p1_z.is_some()) {
self.eval_len2_0()?;
}
if (internalevent.static100 && self.expr_0()?) {
self.streammemory.trigger_0.shift();
}
if internalevent.static100 {
self.eval_violation_0()?;
}
if (((((internalevent.p0_x.is_some() && internalevent.p0_y.is_some()) && internalevent.p0_z.is_some()) && internalevent.p1_x.is_some()) && internalevent.p1_y.is_some()) && internalevent.p1_z.is_some()) {
self.eval_seg_valid_0()?;
}
if (internalevent.static100 && self.expr_0()?) {
self.eval_trigger_0_0()?;
}
if (internalevent.static100 && self.expr_1()?) {
self.streammemory.trigger_1.shift();
}
if ((((((((internalevent.x.is_some() && internalevent.y.is_some()) && internalevent.z.is_some()) && internalevent.p0_x.is_some()) && internalevent.p0_y.is_some()) && internalevent.p0_z.is_some()) && internalevent.p1_x.is_some()) && internalevent.p1_y.is_some()) && internalevent.p1_z.is_some()) {
self.eval_t_raw_0()?;
}
if (internalevent.static100 && self.expr_1()?) {
self.eval_trigger_1_0()?;
}
if ((((((((internalevent.x.is_some() && internalevent.y.is_some()) && internalevent.z.is_some()) && internalevent.p0_x.is_some()) && internalevent.p0_y.is_some()) && internalevent.p0_z.is_some()) && internalevent.p1_x.is_some()) && internalevent.p1_y.is_some()) && internalevent.p1_z.is_some()) {
self.eval_t_0()?;
}
if ((((((((internalevent.x.is_some() && internalevent.y.is_some()) && internalevent.z.is_some()) && internalevent.p0_x.is_some()) && internalevent.p0_y.is_some()) && internalevent.p0_z.is_some()) && internalevent.p1_x.is_some()) && internalevent.p1_y.is_some()) && internalevent.p1_z.is_some()) {
self.eval_c_x_0()?;
}
if ((((((((internalevent.x.is_some() && internalevent.y.is_some()) && internalevent.z.is_some()) && internalevent.p0_x.is_some()) && internalevent.p0_y.is_some()) && internalevent.p0_z.is_some()) && internalevent.p1_x.is_some()) && internalevent.p1_y.is_some()) && internalevent.p1_z.is_some()) {
self.eval_c_y_0()?;
}
if ((((((((internalevent.x.is_some() && internalevent.y.is_some()) && internalevent.z.is_some()) && internalevent.p0_x.is_some()) && internalevent.p0_y.is_some()) && internalevent.p0_z.is_some()) && internalevent.p1_x.is_some()) && internalevent.p1_y.is_some()) && internalevent.p1_z.is_some()) {
self.eval_c_z_0()?;
}
if ((((((((internalevent.x.is_some() && internalevent.y.is_some()) && internalevent.z.is_some()) && internalevent.p0_x.is_some()) && internalevent.p0_y.is_some()) && internalevent.p0_z.is_some()) && internalevent.p1_x.is_some()) && internalevent.p1_y.is_some()) && internalevent.p1_z.is_some()) {
self.eval_cross_track_drift_0()?;
}
let verdict = Verdict::new(self)?;
self.clear()?;
self.clear_activations();
Ok(verdict)
}
}
impl Monitor {
pub(crate) fn x(&self) -> Result<f64, MonitorError> {
Ok(self.streammemory.x.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn y(&self) -> Result<f64, MonitorError> {
Ok(self.streammemory.y.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn z(&self) -> Result<f64, MonitorError> {
Ok(self.streammemory.z.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn p0_x(&self) -> Result<f64, MonitorError> {
Ok(self.streammemory.p0_x.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn p0_y(&self) -> Result<f64, MonitorError> {
Ok(self.streammemory.p0_y.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn p0_z(&self) -> Result<f64, MonitorError> {
Ok(self.streammemory.p0_z.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn p1_x(&self) -> Result<f64, MonitorError> {
Ok(self.streammemory.p1_x.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn p1_y(&self) -> Result<f64, MonitorError> {
Ok(self.streammemory.p1_y.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn p1_z(&self) -> Result<f64, MonitorError> {
Ok(self.streammemory.p1_z.get(0)?.cloned().expect("sync access"))
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
pub(crate) fn len2(&self) -> Result<f64, MonitorError> {
Ok(self.streammemory.len2.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn seg_valid(&self) -> Result<bool, MonitorError> {
Ok(self.streammemory.seg_valid.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn vx(&self) -> Result<f64, MonitorError> {
Ok(self.streammemory.vx.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn vy(&self) -> Result<f64, MonitorError> {
Ok(self.streammemory.vy.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn vz(&self) -> Result<f64, MonitorError> {
Ok(self.streammemory.vz.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn t_raw(&self) -> Result<f64, MonitorError> {
Ok(self.streammemory.t_raw.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn t(&self) -> Result<f64, MonitorError> {
Ok(self.streammemory.t.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn c_x(&self) -> Result<f64, MonitorError> {
Ok(self.streammemory.c_x.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn c_y(&self) -> Result<f64, MonitorError> {
Ok(self.streammemory.c_y.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn c_z(&self) -> Result<f64, MonitorError> {
Ok(self.streammemory.c_z.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn warning(&self) -> Result<bool, MonitorError> {
Ok(self.streammemory.warning.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn violation(&self) -> Result<bool, MonitorError> {
Ok(self.streammemory.violation.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn cross_track_drift_hold(&self) -> Result<Option<f64>, MonitorError> {
Ok(self.streammemory.cross_track_drift.get(0)?.cloned())
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
pub(crate) fn len2_get(&self) -> Result<Option<f64>, MonitorError> {
Ok(if self.streammemory.len2.is_fresh() {
self.streammemory.len2.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn seg_valid_get(&self) -> Result<Option<bool>, MonitorError> {
Ok(if self.streammemory.seg_valid.is_fresh() {
self.streammemory.seg_valid.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn vx_get(&self) -> Result<Option<f64>, MonitorError> {
Ok(if self.streammemory.vx.is_fresh() {
self.streammemory.vx.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn vy_get(&self) -> Result<Option<f64>, MonitorError> {
Ok(if self.streammemory.vy.is_fresh() {
self.streammemory.vy.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn vz_get(&self) -> Result<Option<f64>, MonitorError> {
Ok(if self.streammemory.vz.is_fresh() {
self.streammemory.vz.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn t_raw_get(&self) -> Result<Option<f64>, MonitorError> {
Ok(if self.streammemory.t_raw.is_fresh() {
self.streammemory.t_raw.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn t_get(&self) -> Result<Option<f64>, MonitorError> {
Ok(if self.streammemory.t.is_fresh() {
self.streammemory.t.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn c_x_get(&self) -> Result<Option<f64>, MonitorError> {
Ok(if self.streammemory.c_x.is_fresh() {
self.streammemory.c_x.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn c_y_get(&self) -> Result<Option<f64>, MonitorError> {
Ok(if self.streammemory.c_y.is_fresh() {
self.streammemory.c_y.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn c_z_get(&self) -> Result<Option<f64>, MonitorError> {
Ok(if self.streammemory.c_z.is_fresh() {
self.streammemory.c_z.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn cross_track_drift_get(&self) -> Result<Option<f64>, MonitorError> {
Ok(if self.streammemory.cross_track_drift.is_fresh() {
self.streammemory.cross_track_drift.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn warning_get(&self) -> Result<Option<bool>, MonitorError> {
Ok(if self.streammemory.warning.is_fresh() {
self.streammemory.warning.get(0)?.cloned()
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
pub(crate) fn violation_get(&self) -> Result<Option<bool>, MonitorError> {
Ok(if self.streammemory.violation.is_fresh() {
self.streammemory.violation.get(0)?.cloned()
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
pub(crate) fn eval_x(&mut self,new_value: f64) -> Result<(), MonitorError> {

self.streammemory.x.update(new_value)
}
}
impl Monitor {
pub(crate) fn eval_y(&mut self,new_value: f64) -> Result<(), MonitorError> {

self.streammemory.y.update(new_value)
}
}
impl Monitor {
pub(crate) fn eval_z(&mut self,new_value: f64) -> Result<(), MonitorError> {

self.streammemory.z.update(new_value)
}
}
impl Monitor {
pub(crate) fn eval_p0_x(&mut self,new_value: f64) -> Result<(), MonitorError> {

self.streammemory.p0_x.update(new_value)
}
}
impl Monitor {
pub(crate) fn eval_p0_y(&mut self,new_value: f64) -> Result<(), MonitorError> {

self.streammemory.p0_y.update(new_value)
}
}
impl Monitor {
pub(crate) fn eval_p0_z(&mut self,new_value: f64) -> Result<(), MonitorError> {

self.streammemory.p0_z.update(new_value)
}
}
impl Monitor {
pub(crate) fn eval_p1_x(&mut self,new_value: f64) -> Result<(), MonitorError> {

self.streammemory.p1_x.update(new_value)
}
}
impl Monitor {
pub(crate) fn eval_p1_y(&mut self,new_value: f64) -> Result<(), MonitorError> {

self.streammemory.p1_y.update(new_value)
}
}
impl Monitor {
pub(crate) fn eval_p1_z(&mut self,new_value: f64) -> Result<(), MonitorError> {

self.streammemory.p1_z.update(new_value)
}
}
impl Monitor {
pub(crate) fn eval_dx_0(&mut self) -> Result<(), MonitorError> {
let new_value = (self.p1_x()? - self.p0_x()?);
;
self.streammemory.dx.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_dy_0(&mut self) -> Result<(), MonitorError> {
let new_value = (self.p1_y()? - self.p0_y()?);
;
self.streammemory.dy.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_dz_0(&mut self) -> Result<(), MonitorError> {
let new_value = (self.p1_z()? - self.p0_z()?);
;
self.streammemory.dz.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_len2_0(&mut self) -> Result<(), MonitorError> {
let new_value = (((self.dx()? * self.dx()?) + (self.dy()? * self.dy()?)) + (self.dz()? * self.dz()?));
;
self.streammemory.len2.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_seg_valid_0(&mut self) -> Result<(), MonitorError> {
let new_value = (self.len2()? > 0f64);
;
self.streammemory.seg_valid.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_vx_0(&mut self) -> Result<(), MonitorError> {
let new_value = (self.x()? - self.p0_x()?);
;
self.streammemory.vx.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_vy_0(&mut self) -> Result<(), MonitorError> {
let new_value = (self.y()? - self.p0_y()?);
;
self.streammemory.vy.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_vz_0(&mut self) -> Result<(), MonitorError> {
let new_value = (self.z()? - self.p0_z()?);
;
self.streammemory.vz.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_t_raw_0(&mut self) -> Result<(), MonitorError> {
let new_value = (if self.seg_valid()? { ((((self.vx()? * self.dx()?) + (self.vy()? * self.dy()?)) + (self.vz()? * self.dz()?)) / self.len2()?) } else { 0f64 });
;
self.streammemory.t_raw.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_t_0(&mut self) -> Result<(), MonitorError> {
let new_value = (if (self.t_raw()? < 0f64) { 0f64 } else { (if (self.t_raw()? > 1f64) { 1f64 } else { self.t_raw()? }) });
;
self.streammemory.t.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_c_x_0(&mut self) -> Result<(), MonitorError> {
let new_value = (self.p0_x()? + (self.t()? * self.dx()?));
;
self.streammemory.c_x.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_c_y_0(&mut self) -> Result<(), MonitorError> {
let new_value = (self.p0_y()? + (self.t()? * self.dy()?));
;
self.streammemory.c_y.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_c_z_0(&mut self) -> Result<(), MonitorError> {
let new_value = (self.p0_z()? + (self.t()? * self.dz()?));
;
self.streammemory.c_z.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_cross_track_drift_0(&mut self) -> Result<(), MonitorError> {
let new_value = (((((self.x()? - self.c_x()?) * (self.x()? - self.c_x()?)) + ((self.y()? - self.c_y()?) * (self.y()? - self.c_y()?))) + ((self.z()? - self.c_z()?) * (self.z()? - self.c_z()?)))).sqrt();
;
self.streammemory.cross_track_drift.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_warning_0(&mut self) -> Result<(), MonitorError> {
let new_value = (self.cross_track_drift_hold()?.unwrap_or(0f64) > 0.35f64);
self.windowsmemory.sliding0.accept_value(self.time, new_value);;
self.streammemory.warning.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_trigger_0_0(&mut self) -> Result<(), MonitorError> {
let new_value = "Warning: Cross-track drift > 0.35 m — correcting toward path!".to_owned();
;
self.streammemory.trigger_0.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_violation_0(&mut self) -> Result<(), MonitorError> {
let new_value = self.windowsmemory.sliding0.get_value(self.time).unwrap();
;
self.streammemory.violation.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_trigger_1_0(&mut self) -> Result<(), MonitorError> {
let new_value = "Violation: Path deviation sustained for 2.5s — land nano-quadcopter!".to_owned();
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