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
x_drift: StreamBuffer<f64, 1>,
y_drift: StreamBuffer<f64, 1>,
z_drift: StreamBuffer<f64, 1>,
pitch: StreamBuffer<f64, 1>,
roll: StreamBuffer<f64, 1>,
yaw: StreamBuffer<f64, 1>,
multi_ranger_x_drift: StreamBuffer<f64, 1>,
multi_ranger_y_drift: StreamBuffer<f64, 1>,
multi_ranger_z_drift: StreamBuffer<f64, 1>,
x: StreamBuffer<f64, 1>,
y: StreamBuffer<f64, 1>,
z: StreamBuffer<f64, 1>,
waypoint_x: StreamBuffer<f64, 1>,
waypoint_y: StreamBuffer<f64, 1>,
waypoint_z: StreamBuffer<f64, 1>,
abs_pitch: StreamBuffer<f64, 1>,
abs_roll: StreamBuffer<f64, 1>,
abs_yaw: StreamBuffer<f64, 1>,
x_drift_pos_exceeded: StreamBuffer<bool, 1>,
trigger_0: StreamBuffer<String, 1>,
x_drift_neg_exceeded: StreamBuffer<bool, 1>,
trigger_1: StreamBuffer<String, 1>,
y_drift_pos_exceeded: StreamBuffer<bool, 1>,
trigger_2: StreamBuffer<String, 1>,
y_drift_neg_exceeded: StreamBuffer<bool, 1>,
trigger_3: StreamBuffer<String, 1>,
z_drift_pos_exceeded: StreamBuffer<bool, 1>,
trigger_4: StreamBuffer<String, 1>,
z_drift_neg_exceeded: StreamBuffer<bool, 1>,
trigger_5: StreamBuffer<String, 1>,
multi_ranger_x_drift_pos_exceeded: StreamBuffer<bool, 1>,
trigger_6: StreamBuffer<String, 1>,
multi_ranger_x_drift_neg_exceeded: StreamBuffer<bool, 1>,
trigger_7: StreamBuffer<String, 1>,
multi_ranger_y_drift_pos_exceeded: StreamBuffer<bool, 1>,
trigger_8: StreamBuffer<String, 1>,
multi_ranger_y_drift_neg_exceeded: StreamBuffer<bool, 1>,
trigger_9: StreamBuffer<String, 1>,
multi_ranger_z_drift_pos_exceeded: StreamBuffer<bool, 1>,
trigger_10: StreamBuffer<String, 1>,
multi_ranger_z_drift_neg_exceeded: StreamBuffer<bool, 1>,
trigger_11: StreamBuffer<String, 1>,
pitch_exceeded: StreamBuffer<bool, 1>,
trigger_12: StreamBuffer<String, 1>,
roll_exceeded: StreamBuffer<bool, 1>,
trigger_13: StreamBuffer<String, 1>,
yaw_exceeded: StreamBuffer<bool, 1>,
trigger_14: StreamBuffer<String, 1>,
dist_to_waypoint: StreamBuffer<f64, 2>,
prev_dist_to_waypoint: StreamBuffer<f64, 1>,
step_closure: StreamBuffer<f64, 1>,
reached_waypoint: StreamBuffer<bool, 1>,
min_closure_2s: StreamBuffer<f64, 1>,
progressing_2s: StreamBuffer<bool, 1>,
min_closure_5s: StreamBuffer<f64, 1>,
progressing_5s: StreamBuffer<bool, 1>,
trigger_15: StreamBuffer<String, 1>,
trigger_16: StreamBuffer<String, 1>,
trigger_17: StreamBuffer<String, 1>,
}
impl StreamMemory {
fn new(start_time: core::time::Duration) -> Self {
Self {
x_drift: StreamBuffer::new(),
y_drift: StreamBuffer::new(),
z_drift: StreamBuffer::new(),
pitch: StreamBuffer::new(),
roll: StreamBuffer::new(),
yaw: StreamBuffer::new(),
multi_ranger_x_drift: StreamBuffer::new(),
multi_ranger_y_drift: StreamBuffer::new(),
multi_ranger_z_drift: StreamBuffer::new(),
x: StreamBuffer::new(),
y: StreamBuffer::new(),
z: StreamBuffer::new(),
waypoint_x: StreamBuffer::new(),
waypoint_y: StreamBuffer::new(),
waypoint_z: StreamBuffer::new(),
abs_pitch: StreamBuffer::new(),
abs_roll: StreamBuffer::new(),
abs_yaw: StreamBuffer::new(),
x_drift_pos_exceeded: StreamBuffer::new(),
trigger_0: StreamBuffer::new(),
x_drift_neg_exceeded: StreamBuffer::new(),
trigger_1: StreamBuffer::new(),
y_drift_pos_exceeded: StreamBuffer::new(),
trigger_2: StreamBuffer::new(),
y_drift_neg_exceeded: StreamBuffer::new(),
trigger_3: StreamBuffer::new(),
z_drift_pos_exceeded: StreamBuffer::new(),
trigger_4: StreamBuffer::new(),
z_drift_neg_exceeded: StreamBuffer::new(),
trigger_5: StreamBuffer::new(),
multi_ranger_x_drift_pos_exceeded: StreamBuffer::new(),
trigger_6: StreamBuffer::new(),
multi_ranger_x_drift_neg_exceeded: StreamBuffer::new(),
trigger_7: StreamBuffer::new(),
multi_ranger_y_drift_pos_exceeded: StreamBuffer::new(),
trigger_8: StreamBuffer::new(),
multi_ranger_y_drift_neg_exceeded: StreamBuffer::new(),
trigger_9: StreamBuffer::new(),
multi_ranger_z_drift_pos_exceeded: StreamBuffer::new(),
trigger_10: StreamBuffer::new(),
multi_ranger_z_drift_neg_exceeded: StreamBuffer::new(),
trigger_11: StreamBuffer::new(),
pitch_exceeded: StreamBuffer::new(),
trigger_12: StreamBuffer::new(),
roll_exceeded: StreamBuffer::new(),
trigger_13: StreamBuffer::new(),
yaw_exceeded: StreamBuffer::new(),
trigger_14: StreamBuffer::new(),
dist_to_waypoint: StreamBuffer::new(),
prev_dist_to_waypoint: StreamBuffer::new(),
step_closure: StreamBuffer::new(),
reached_waypoint: StreamBuffer::new(),
min_closure_2s: StreamBuffer::new(),
progressing_2s: StreamBuffer::new(),
min_closure_5s: StreamBuffer::new(),
progressing_5s: StreamBuffer::new(),
trigger_15: StreamBuffer::new(),
trigger_16: StreamBuffer::new(),
trigger_17: StreamBuffer::new()
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
X_drift,
Y_drift,
Z_drift,
Pitch,
Roll,
Yaw,
Multi_ranger_x_drift,
Multi_ranger_y_drift,
Multi_ranger_z_drift,
X,
Y,
Z,
Waypoint_x,
Waypoint_y,
Waypoint_z,
Abs_pitch,
Abs_roll,
Abs_yaw,
X_drift_pos_exceeded,
Trigger_0,
X_drift_neg_exceeded,
Trigger_1,
Y_drift_pos_exceeded,
Trigger_2,
Y_drift_neg_exceeded,
Trigger_3,
Z_drift_pos_exceeded,
Trigger_4,
Z_drift_neg_exceeded,
Trigger_5,
Multi_ranger_x_drift_pos_exceeded,
Trigger_6,
Multi_ranger_x_drift_neg_exceeded,
Trigger_7,
Multi_ranger_y_drift_pos_exceeded,
Trigger_8,
Multi_ranger_y_drift_neg_exceeded,
Trigger_9,
Multi_ranger_z_drift_pos_exceeded,
Trigger_10,
Multi_ranger_z_drift_neg_exceeded,
Trigger_11,
Pitch_exceeded,
Trigger_12,
Roll_exceeded,
Trigger_13,
Yaw_exceeded,
Trigger_14,
Dist_to_waypoint,
Prev_dist_to_waypoint,
Step_closure,
Reached_waypoint,
Min_closure_2s,
Progressing_2s,
Min_closure_5s,
Progressing_5s,
Trigger_15,
Trigger_16,
Trigger_17,
}
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
enum Deadline {
Static1000,
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
time: core::time::Duration,
deadline: Deadline,
}
impl State {
fn new_after(deadline: Deadline, time: core::time::Duration) -> Self {
match deadline {
            Deadline::Static1000 => State {
        time: time + core::time::Duration::new(1, 0),
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

            time: start_time + core::time::Duration::new(1, 0),
            deadline: Deadline::Static1000
        
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
Deadline::Static1000 => {current_event.static1000 = true;}
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
sliding0: SlidingWindowBuffer<MinAggregation<f64>, 2, false>,
sliding1: SlidingWindowBuffer<MinAggregation<f64>, 5, false>,
}
impl WindowsMemory {
fn new(start_time: core::time::Duration) -> Self {
Self {
sliding0: SlidingWindowBuffer::new(start_time, core::time::Duration::new(1, 0)),
sliding1: SlidingWindowBuffer::new(start_time, core::time::Duration::new(1, 0))
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
struct MinAggregation<BucketType>(PhantomData<BucketType>);
impl<Value: Copy + Debug + PartialOrd> WindowAggregation for MinAggregation<Value> {
    type BucketType = Option<Value>;
    type ResultType = Option<Value>;
    type ValueType = Value;

    fn aggregate(value1: Self::BucketType, value2: Self::BucketType) -> Self::BucketType {
        match (value1, value2) {
            (None, v) | (v, None) => v,
            (Some(v1), Some(v2)) if v1 > v2 => Some(v2),
            (Some(v1), Some(v2)) => Some(v1),
        }
    }
    fn map(value: Self::ValueType, _time: Duration) -> Self::BucketType {
        Some(value)
    }

    fn lower(value: Self::BucketType) -> Self::ResultType {
        value
    }

    fn initial_value(_time: Duration) -> Self::BucketType {
        None
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
z: Option<f64>,
y_drift: Option<f64>,
multi_ranger_x_drift: Option<f64>,
multi_ranger_y_drift: Option<f64>,
multi_ranger_z_drift: Option<f64>,
roll: Option<f64>,
waypoint_z: Option<f64>,
z_drift: Option<f64>,
yaw: Option<f64>,
x: Option<f64>,
y: Option<f64>,
pitch: Option<f64>,
waypoint_y: Option<f64>,
x_drift: Option<f64>,
waypoint_x: Option<f64>,
}
struct InternalEvent {
x_drift: Option<f64>,
y_drift: Option<f64>,
z_drift: Option<f64>,
pitch: Option<f64>,
roll: Option<f64>,
yaw: Option<f64>,
multi_ranger_x_drift: Option<f64>,
multi_ranger_y_drift: Option<f64>,
multi_ranger_z_drift: Option<f64>,
x: Option<f64>,
y: Option<f64>,
z: Option<f64>,
waypoint_x: Option<f64>,
waypoint_y: Option<f64>,
waypoint_z: Option<f64>,
static1000: bool,
time: core::time::Duration,
}
impl InternalEvent {
fn from_event(time: core::time::Duration, event: Event) -> InternalEvent {
Self {
x_drift: event.x_drift
,y_drift: event.y_drift
,z_drift: event.z_drift
,pitch: event.pitch
,roll: event.roll
,yaw: event.yaw
,multi_ranger_x_drift: event.multi_ranger_x_drift
,multi_ranger_y_drift: event.multi_ranger_y_drift
,multi_ranger_z_drift: event.multi_ranger_z_drift
,x: event.x
,y: event.y
,z: event.z
,waypoint_x: event.waypoint_x
,waypoint_y: event.waypoint_y
,waypoint_z: event.waypoint_z
,static1000: false
,time: time
}
}
}
impl InternalEvent {
fn empty(time: core::time::Duration) -> Self {
Self { x_drift: None,
y_drift: None,
z_drift: None,
pitch: None,
roll: None,
yaw: None,
multi_ranger_x_drift: None,
multi_ranger_y_drift: None,
multi_ranger_z_drift: None,
x: None,
y: None,
z: None,
waypoint_x: None,
waypoint_y: None,
waypoint_z: None,
static1000: false,
time: time }
}
}
pub struct Verdict {
abs_pitch: Option<f64>,
abs_roll: Option<f64>,
abs_yaw: Option<f64>,
x_drift_pos_exceeded: Option<bool>,
trigger_0: Option<String>,
x_drift_neg_exceeded: Option<bool>,
trigger_1: Option<String>,
y_drift_pos_exceeded: Option<bool>,
trigger_2: Option<String>,
y_drift_neg_exceeded: Option<bool>,
trigger_3: Option<String>,
z_drift_pos_exceeded: Option<bool>,
trigger_4: Option<String>,
z_drift_neg_exceeded: Option<bool>,
trigger_5: Option<String>,
multi_ranger_x_drift_pos_exceeded: Option<bool>,
trigger_6: Option<String>,
multi_ranger_x_drift_neg_exceeded: Option<bool>,
trigger_7: Option<String>,
multi_ranger_y_drift_pos_exceeded: Option<bool>,
trigger_8: Option<String>,
multi_ranger_y_drift_neg_exceeded: Option<bool>,
trigger_9: Option<String>,
multi_ranger_z_drift_pos_exceeded: Option<bool>,
trigger_10: Option<String>,
multi_ranger_z_drift_neg_exceeded: Option<bool>,
trigger_11: Option<String>,
pitch_exceeded: Option<bool>,
trigger_12: Option<String>,
roll_exceeded: Option<bool>,
trigger_13: Option<String>,
yaw_exceeded: Option<bool>,
trigger_14: Option<String>,
dist_to_waypoint: Option<f64>,
prev_dist_to_waypoint: Option<f64>,
step_closure: Option<f64>,
reached_waypoint: Option<bool>,
min_closure_2s: Option<f64>,
progressing_2s: Option<bool>,
min_closure_5s: Option<f64>,
progressing_5s: Option<bool>,
trigger_15: Option<String>,
trigger_16: Option<String>,
trigger_17: Option<String>,
time: core::time::Duration,
}
impl Verdict {
fn new(monitor: &mut Monitor) -> Result<Verdict, MonitorError> {
Ok(Self {
abs_pitch: monitor.abs_pitch_get()?,
abs_roll: monitor.abs_roll_get()?,
abs_yaw: monitor.abs_yaw_get()?,
x_drift_pos_exceeded: monitor.x_drift_pos_exceeded_get()?,
trigger_0: monitor.trigger_0_get()?,
x_drift_neg_exceeded: monitor.x_drift_neg_exceeded_get()?,
trigger_1: monitor.trigger_1_get()?,
y_drift_pos_exceeded: monitor.y_drift_pos_exceeded_get()?,
trigger_2: monitor.trigger_2_get()?,
y_drift_neg_exceeded: monitor.y_drift_neg_exceeded_get()?,
trigger_3: monitor.trigger_3_get()?,
z_drift_pos_exceeded: monitor.z_drift_pos_exceeded_get()?,
trigger_4: monitor.trigger_4_get()?,
z_drift_neg_exceeded: monitor.z_drift_neg_exceeded_get()?,
trigger_5: monitor.trigger_5_get()?,
multi_ranger_x_drift_pos_exceeded: monitor.multi_ranger_x_drift_pos_exceeded_get()?,
trigger_6: monitor.trigger_6_get()?,
multi_ranger_x_drift_neg_exceeded: monitor.multi_ranger_x_drift_neg_exceeded_get()?,
trigger_7: monitor.trigger_7_get()?,
multi_ranger_y_drift_pos_exceeded: monitor.multi_ranger_y_drift_pos_exceeded_get()?,
trigger_8: monitor.trigger_8_get()?,
multi_ranger_y_drift_neg_exceeded: monitor.multi_ranger_y_drift_neg_exceeded_get()?,
trigger_9: monitor.trigger_9_get()?,
multi_ranger_z_drift_pos_exceeded: monitor.multi_ranger_z_drift_pos_exceeded_get()?,
trigger_10: monitor.trigger_10_get()?,
multi_ranger_z_drift_neg_exceeded: monitor.multi_ranger_z_drift_neg_exceeded_get()?,
trigger_11: monitor.trigger_11_get()?,
pitch_exceeded: monitor.pitch_exceeded_get()?,
trigger_12: monitor.trigger_12_get()?,
roll_exceeded: monitor.roll_exceeded_get()?,
trigger_13: monitor.trigger_13_get()?,
yaw_exceeded: monitor.yaw_exceeded_get()?,
trigger_14: monitor.trigger_14_get()?,
dist_to_waypoint: monitor.dist_to_waypoint_get()?,
prev_dist_to_waypoint: monitor.prev_dist_to_waypoint_get()?,
step_closure: monitor.step_closure_get()?,
reached_waypoint: monitor.reached_waypoint_get()?,
min_closure_2s: monitor.min_closure_2s_get()?,
progressing_2s: monitor.progressing_2s_get()?,
min_closure_5s: monitor.min_closure_5s_get()?,
progressing_5s: monitor.progressing_5s_get()?,
trigger_15: monitor.trigger_15_get()?,
trigger_16: monitor.trigger_16_get()?,
trigger_17: monitor.trigger_17_get()?,
time: monitor.time
})
}
}

impl Display for Verdict {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{},", self.abs_pitch.as_ref().map(|v|v.to_string()).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.abs_roll.as_ref().map(|v|v.to_string()).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.abs_yaw.as_ref().map(|v|v.to_string()).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.x_drift_pos_exceeded.as_ref().map(|v|v.to_string()).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.trigger_0.as_ref().map(|v|format!("\"{v}\"")).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.x_drift_neg_exceeded.as_ref().map(|v|v.to_string()).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.trigger_1.as_ref().map(|v|format!("\"{v}\"")).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.y_drift_pos_exceeded.as_ref().map(|v|v.to_string()).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.trigger_2.as_ref().map(|v|format!("\"{v}\"")).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.y_drift_neg_exceeded.as_ref().map(|v|v.to_string()).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.trigger_3.as_ref().map(|v|format!("\"{v}\"")).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.z_drift_pos_exceeded.as_ref().map(|v|v.to_string()).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.trigger_4.as_ref().map(|v|format!("\"{v}\"")).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.z_drift_neg_exceeded.as_ref().map(|v|v.to_string()).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.trigger_5.as_ref().map(|v|format!("\"{v}\"")).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.multi_ranger_x_drift_pos_exceeded.as_ref().map(|v|v.to_string()).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.trigger_6.as_ref().map(|v|format!("\"{v}\"")).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.multi_ranger_x_drift_neg_exceeded.as_ref().map(|v|v.to_string()).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.trigger_7.as_ref().map(|v|format!("\"{v}\"")).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.multi_ranger_y_drift_pos_exceeded.as_ref().map(|v|v.to_string()).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.trigger_8.as_ref().map(|v|format!("\"{v}\"")).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.multi_ranger_y_drift_neg_exceeded.as_ref().map(|v|v.to_string()).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.trigger_9.as_ref().map(|v|format!("\"{v}\"")).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.multi_ranger_z_drift_pos_exceeded.as_ref().map(|v|v.to_string()).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.trigger_10.as_ref().map(|v|format!("\"{v}\"")).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.multi_ranger_z_drift_neg_exceeded.as_ref().map(|v|v.to_string()).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.trigger_11.as_ref().map(|v|format!("\"{v}\"")).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.pitch_exceeded.as_ref().map(|v|v.to_string()).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.trigger_12.as_ref().map(|v|format!("\"{v}\"")).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.roll_exceeded.as_ref().map(|v|v.to_string()).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.trigger_13.as_ref().map(|v|format!("\"{v}\"")).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.yaw_exceeded.as_ref().map(|v|v.to_string()).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.trigger_14.as_ref().map(|v|format!("\"{v}\"")).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.dist_to_waypoint.as_ref().map(|v|v.to_string()).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.prev_dist_to_waypoint.as_ref().map(|v|v.to_string()).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.step_closure.as_ref().map(|v|v.to_string()).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.reached_waypoint.as_ref().map(|v|v.to_string()).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.min_closure_2s.as_ref().map(|v|v.to_string()).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.progressing_2s.as_ref().map(|v|v.to_string()).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.min_closure_5s.as_ref().map(|v|v.to_string()).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.progressing_5s.as_ref().map(|v|v.to_string()).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.trigger_15.as_ref().map(|v|format!("\"{v}\"")).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.trigger_16.as_ref().map(|v|format!("\"{v}\"")).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.trigger_17.as_ref().map(|v|format!("\"{v}\"")).unwrap_or_else(||"#".into()))?;
		writeln!(f, "{}", self.time.as_secs_f64())
        }
    }
impl Monitor {
fn expr_0(&mut self) -> Result<bool, MonitorError> {
Ok(self.x_drift_pos_exceeded()?)
}
}
impl Monitor {
fn expr_1(&mut self) -> Result<bool, MonitorError> {
Ok(self.x_drift_neg_exceeded()?)
}
}
impl Monitor {
fn expr_2(&mut self) -> Result<bool, MonitorError> {
Ok(self.y_drift_pos_exceeded()?)
}
}
impl Monitor {
fn expr_3(&mut self) -> Result<bool, MonitorError> {
Ok(self.y_drift_neg_exceeded()?)
}
}
impl Monitor {
fn expr_4(&mut self) -> Result<bool, MonitorError> {
Ok(self.z_drift_pos_exceeded()?)
}
}
impl Monitor {
fn expr_5(&mut self) -> Result<bool, MonitorError> {
Ok(self.z_drift_neg_exceeded()?)
}
}
impl Monitor {
fn expr_6(&mut self) -> Result<bool, MonitorError> {
Ok(self.multi_ranger_x_drift_pos_exceeded()?)
}
}
impl Monitor {
fn expr_7(&mut self) -> Result<bool, MonitorError> {
Ok(self.multi_ranger_x_drift_neg_exceeded()?)
}
}
impl Monitor {
fn expr_8(&mut self) -> Result<bool, MonitorError> {
Ok(self.multi_ranger_y_drift_pos_exceeded()?)
}
}
impl Monitor {
fn expr_9(&mut self) -> Result<bool, MonitorError> {
Ok(self.multi_ranger_y_drift_neg_exceeded()?)
}
}
impl Monitor {
fn expr_10(&mut self) -> Result<bool, MonitorError> {
Ok(self.multi_ranger_z_drift_pos_exceeded()?)
}
}
impl Monitor {
fn expr_11(&mut self) -> Result<bool, MonitorError> {
Ok(self.multi_ranger_z_drift_neg_exceeded()?)
}
}
impl Monitor {
fn expr_12(&mut self) -> Result<bool, MonitorError> {
Ok(self.pitch_exceeded()?)
}
}
impl Monitor {
fn expr_13(&mut self) -> Result<bool, MonitorError> {
Ok(self.roll_exceeded()?)
}
}
impl Monitor {
fn expr_14(&mut self) -> Result<bool, MonitorError> {
Ok(self.yaw_exceeded()?)
}
}
impl Monitor {
fn expr_15(&mut self) -> Result<bool, MonitorError> {
Ok(self.reached_waypoint()?)
}
}
impl Monitor {
fn expr_16(&mut self) -> Result<bool, MonitorError> {
Ok(((!self.progressing_2s()?) && (!self.reached_waypoint_hold()?.unwrap_or(false))))
}
}
impl Monitor {
fn expr_17(&mut self) -> Result<bool, MonitorError> {
Ok(((!self.progressing_5s()?) && (!self.reached_waypoint_hold()?.unwrap_or(false))))
}
}
impl Monitor {
fn clear_activations(&mut self) -> () {
self.streammemory.x_drift.clear_activation();
self.streammemory.y_drift.clear_activation();
self.streammemory.z_drift.clear_activation();
self.streammemory.pitch.clear_activation();
self.streammemory.roll.clear_activation();
self.streammemory.yaw.clear_activation();
self.streammemory.multi_ranger_x_drift.clear_activation();
self.streammemory.multi_ranger_y_drift.clear_activation();
self.streammemory.multi_ranger_z_drift.clear_activation();
self.streammemory.x.clear_activation();
self.streammemory.y.clear_activation();
self.streammemory.z.clear_activation();
self.streammemory.waypoint_x.clear_activation();
self.streammemory.waypoint_y.clear_activation();
self.streammemory.waypoint_z.clear_activation();
self.streammemory.abs_pitch.clear_activation();
self.streammemory.abs_roll.clear_activation();
self.streammemory.abs_yaw.clear_activation();
self.streammemory.x_drift_pos_exceeded.clear_activation();
self.streammemory.trigger_0.clear_activation();
self.streammemory.x_drift_neg_exceeded.clear_activation();
self.streammemory.trigger_1.clear_activation();
self.streammemory.y_drift_pos_exceeded.clear_activation();
self.streammemory.trigger_2.clear_activation();
self.streammemory.y_drift_neg_exceeded.clear_activation();
self.streammemory.trigger_3.clear_activation();
self.streammemory.z_drift_pos_exceeded.clear_activation();
self.streammemory.trigger_4.clear_activation();
self.streammemory.z_drift_neg_exceeded.clear_activation();
self.streammemory.trigger_5.clear_activation();
self.streammemory.multi_ranger_x_drift_pos_exceeded.clear_activation();
self.streammemory.trigger_6.clear_activation();
self.streammemory.multi_ranger_x_drift_neg_exceeded.clear_activation();
self.streammemory.trigger_7.clear_activation();
self.streammemory.multi_ranger_y_drift_pos_exceeded.clear_activation();
self.streammemory.trigger_8.clear_activation();
self.streammemory.multi_ranger_y_drift_neg_exceeded.clear_activation();
self.streammemory.trigger_9.clear_activation();
self.streammemory.multi_ranger_z_drift_pos_exceeded.clear_activation();
self.streammemory.trigger_10.clear_activation();
self.streammemory.multi_ranger_z_drift_neg_exceeded.clear_activation();
self.streammemory.trigger_11.clear_activation();
self.streammemory.pitch_exceeded.clear_activation();
self.streammemory.trigger_12.clear_activation();
self.streammemory.roll_exceeded.clear_activation();
self.streammemory.trigger_13.clear_activation();
self.streammemory.yaw_exceeded.clear_activation();
self.streammemory.trigger_14.clear_activation();
self.streammemory.dist_to_waypoint.clear_activation();
self.streammemory.prev_dist_to_waypoint.clear_activation();
self.streammemory.step_closure.clear_activation();
self.streammemory.reached_waypoint.clear_activation();
self.streammemory.min_closure_2s.clear_activation();
self.streammemory.progressing_2s.clear_activation();
self.streammemory.min_closure_5s.clear_activation();
self.streammemory.progressing_5s.clear_activation();
self.streammemory.trigger_15.clear_activation();
self.streammemory.trigger_16.clear_activation();
self.streammemory.trigger_17.clear_activation();
}
}
impl Monitor {
fn cycle(&mut self,internalevent: InternalEvent) -> Result<Verdict, MonitorError> {
self.time = internalevent.time;
if internalevent.x_drift.is_some() {
self.streammemory.x_drift.shift();
self.eval_x_drift(internalevent.x_drift.expect("Checked the existence with the guard"))?;
}
if internalevent.y_drift.is_some() {
self.streammemory.y_drift.shift();
self.eval_y_drift(internalevent.y_drift.expect("Checked the existence with the guard"))?;
}
if internalevent.z_drift.is_some() {
self.streammemory.z_drift.shift();
self.eval_z_drift(internalevent.z_drift.expect("Checked the existence with the guard"))?;
}
if internalevent.pitch.is_some() {
self.streammemory.pitch.shift();
self.eval_pitch(internalevent.pitch.expect("Checked the existence with the guard"))?;
}
if internalevent.roll.is_some() {
self.streammemory.roll.shift();
self.eval_roll(internalevent.roll.expect("Checked the existence with the guard"))?;
}
if internalevent.yaw.is_some() {
self.streammemory.yaw.shift();
self.eval_yaw(internalevent.yaw.expect("Checked the existence with the guard"))?;
}
if internalevent.multi_ranger_x_drift.is_some() {
self.streammemory.multi_ranger_x_drift.shift();
self.eval_multi_ranger_x_drift(internalevent.multi_ranger_x_drift.expect("Checked the existence with the guard"))?;
}
if internalevent.multi_ranger_y_drift.is_some() {
self.streammemory.multi_ranger_y_drift.shift();
self.eval_multi_ranger_y_drift(internalevent.multi_ranger_y_drift.expect("Checked the existence with the guard"))?;
}
if internalevent.multi_ranger_z_drift.is_some() {
self.streammemory.multi_ranger_z_drift.shift();
self.eval_multi_ranger_z_drift(internalevent.multi_ranger_z_drift.expect("Checked the existence with the guard"))?;
}
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
if internalevent.waypoint_x.is_some() {
self.streammemory.waypoint_x.shift();
self.eval_waypoint_x(internalevent.waypoint_x.expect("Checked the existence with the guard"))?;
}
if internalevent.waypoint_y.is_some() {
self.streammemory.waypoint_y.shift();
self.eval_waypoint_y(internalevent.waypoint_y.expect("Checked the existence with the guard"))?;
}
if internalevent.waypoint_z.is_some() {
self.streammemory.waypoint_z.shift();
self.eval_waypoint_z(internalevent.waypoint_z.expect("Checked the existence with the guard"))?;
}
if internalevent.pitch.is_some() {
self.streammemory.abs_pitch.shift();
}
if internalevent.roll.is_some() {
self.streammemory.abs_roll.shift();
}
if internalevent.yaw.is_some() {
self.streammemory.abs_yaw.shift();
}
if internalevent.x_drift.is_some() {
self.streammemory.x_drift_pos_exceeded.shift();
}
if internalevent.x_drift.is_some() {
self.streammemory.x_drift_neg_exceeded.shift();
}
if internalevent.y_drift.is_some() {
self.streammemory.y_drift_pos_exceeded.shift();
}
if internalevent.y_drift.is_some() {
self.streammemory.y_drift_neg_exceeded.shift();
}
if internalevent.z_drift.is_some() {
self.streammemory.z_drift_pos_exceeded.shift();
}
if internalevent.z_drift.is_some() {
self.streammemory.z_drift_neg_exceeded.shift();
}
if internalevent.multi_ranger_x_drift.is_some() {
self.streammemory.multi_ranger_x_drift_pos_exceeded.shift();
}
if internalevent.multi_ranger_x_drift.is_some() {
self.streammemory.multi_ranger_x_drift_neg_exceeded.shift();
}
if internalevent.multi_ranger_y_drift.is_some() {
self.streammemory.multi_ranger_y_drift_pos_exceeded.shift();
}
if internalevent.multi_ranger_y_drift.is_some() {
self.streammemory.multi_ranger_y_drift_neg_exceeded.shift();
}
if internalevent.multi_ranger_z_drift.is_some() {
self.streammemory.multi_ranger_z_drift_pos_exceeded.shift();
}
if internalevent.multi_ranger_z_drift.is_some() {
self.streammemory.multi_ranger_z_drift_neg_exceeded.shift();
}
if internalevent.pitch.is_some() {
self.streammemory.pitch_exceeded.shift();
}
if internalevent.roll.is_some() {
self.streammemory.roll_exceeded.shift();
}
if internalevent.yaw.is_some() {
self.streammemory.yaw_exceeded.shift();
}
if (((((internalevent.x.is_some() && internalevent.y.is_some()) && internalevent.z.is_some()) && internalevent.waypoint_x.is_some()) && internalevent.waypoint_y.is_some()) && internalevent.waypoint_z.is_some()) {
self.streammemory.dist_to_waypoint.shift();
}
if (((((internalevent.x.is_some() && internalevent.y.is_some()) && internalevent.z.is_some()) && internalevent.waypoint_x.is_some()) && internalevent.waypoint_y.is_some()) && internalevent.waypoint_z.is_some()) {
self.streammemory.prev_dist_to_waypoint.shift();
}
if (((((internalevent.x.is_some() && internalevent.y.is_some()) && internalevent.z.is_some()) && internalevent.waypoint_x.is_some()) && internalevent.waypoint_y.is_some()) && internalevent.waypoint_z.is_some()) {
self.streammemory.step_closure.shift();
}
if (((((internalevent.x.is_some() && internalevent.y.is_some()) && internalevent.z.is_some()) && internalevent.waypoint_x.is_some()) && internalevent.waypoint_y.is_some()) && internalevent.waypoint_z.is_some()) {
self.streammemory.reached_waypoint.shift();
}
if internalevent.static1000 {
self.streammemory.min_closure_2s.shift();
}
if internalevent.static1000 {
self.streammemory.progressing_2s.shift();
}
if internalevent.static1000 {
self.streammemory.min_closure_5s.shift();
}
if internalevent.static1000 {
self.streammemory.progressing_5s.shift();
}
if internalevent.pitch.is_some() {
self.eval_abs_pitch_0()?;
}
if internalevent.roll.is_some() {
self.eval_abs_roll_0()?;
}
if internalevent.yaw.is_some() {
self.eval_abs_yaw_0()?;
}
if internalevent.x_drift.is_some() {
self.eval_x_drift_pos_exceeded_0()?;
}
if internalevent.x_drift.is_some() {
self.eval_x_drift_neg_exceeded_0()?;
}
if internalevent.y_drift.is_some() {
self.eval_y_drift_pos_exceeded_0()?;
}
if internalevent.y_drift.is_some() {
self.eval_y_drift_neg_exceeded_0()?;
}
if internalevent.z_drift.is_some() {
self.eval_z_drift_pos_exceeded_0()?;
}
if internalevent.z_drift.is_some() {
self.eval_z_drift_neg_exceeded_0()?;
}
if internalevent.multi_ranger_x_drift.is_some() {
self.eval_multi_ranger_x_drift_pos_exceeded_0()?;
}
if internalevent.multi_ranger_x_drift.is_some() {
self.eval_multi_ranger_x_drift_neg_exceeded_0()?;
}
if internalevent.multi_ranger_y_drift.is_some() {
self.eval_multi_ranger_y_drift_pos_exceeded_0()?;
}
if internalevent.multi_ranger_y_drift.is_some() {
self.eval_multi_ranger_y_drift_neg_exceeded_0()?;
}
if internalevent.multi_ranger_z_drift.is_some() {
self.eval_multi_ranger_z_drift_pos_exceeded_0()?;
}
if internalevent.multi_ranger_z_drift.is_some() {
self.eval_multi_ranger_z_drift_neg_exceeded_0()?;
}
if (((((internalevent.x.is_some() && internalevent.y.is_some()) && internalevent.z.is_some()) && internalevent.waypoint_x.is_some()) && internalevent.waypoint_y.is_some()) && internalevent.waypoint_z.is_some()) {
self.eval_dist_to_waypoint_0()?;
}
if (((((internalevent.x.is_some() && internalevent.y.is_some()) && internalevent.z.is_some()) && internalevent.waypoint_x.is_some()) && internalevent.waypoint_y.is_some()) && internalevent.waypoint_z.is_some()) {
self.eval_prev_dist_to_waypoint_0()?;
}
if internalevent.static1000 {
self.eval_min_closure_2s_0()?;
}
if internalevent.static1000 {
self.eval_min_closure_5s_0()?;
}
if (internalevent.x_drift.is_some() && self.expr_0()?) {
self.streammemory.trigger_0.shift();
}
if (internalevent.x_drift.is_some() && self.expr_1()?) {
self.streammemory.trigger_1.shift();
}
if (internalevent.y_drift.is_some() && self.expr_2()?) {
self.streammemory.trigger_2.shift();
}
if (internalevent.y_drift.is_some() && self.expr_3()?) {
self.streammemory.trigger_3.shift();
}
if (internalevent.z_drift.is_some() && self.expr_4()?) {
self.streammemory.trigger_4.shift();
}
if (internalevent.z_drift.is_some() && self.expr_5()?) {
self.streammemory.trigger_5.shift();
}
if (internalevent.multi_ranger_x_drift.is_some() && self.expr_6()?) {
self.streammemory.trigger_6.shift();
}
if (internalevent.multi_ranger_x_drift.is_some() && self.expr_7()?) {
self.streammemory.trigger_7.shift();
}
if (internalevent.multi_ranger_y_drift.is_some() && self.expr_8()?) {
self.streammemory.trigger_8.shift();
}
if (internalevent.multi_ranger_y_drift.is_some() && self.expr_9()?) {
self.streammemory.trigger_9.shift();
}
if (internalevent.multi_ranger_z_drift.is_some() && self.expr_10()?) {
self.streammemory.trigger_10.shift();
}
if (internalevent.multi_ranger_z_drift.is_some() && self.expr_11()?) {
self.streammemory.trigger_11.shift();
}
if internalevent.pitch.is_some() {
self.eval_pitch_exceeded_0()?;
}
if internalevent.roll.is_some() {
self.eval_roll_exceeded_0()?;
}
if internalevent.yaw.is_some() {
self.eval_yaw_exceeded_0()?;
}
if (((((internalevent.x.is_some() && internalevent.y.is_some()) && internalevent.z.is_some()) && internalevent.waypoint_x.is_some()) && internalevent.waypoint_y.is_some()) && internalevent.waypoint_z.is_some()) {
self.eval_step_closure_0()?;
}
if (((((internalevent.x.is_some() && internalevent.y.is_some()) && internalevent.z.is_some()) && internalevent.waypoint_x.is_some()) && internalevent.waypoint_y.is_some()) && internalevent.waypoint_z.is_some()) {
self.eval_reached_waypoint_0()?;
}
if internalevent.static1000 {
self.eval_progressing_2s_0()?;
}
if internalevent.static1000 {
self.eval_progressing_5s_0()?;
}
if (internalevent.x_drift.is_some() && self.expr_0()?) {
self.eval_trigger_0_0()?;
}
if (internalevent.x_drift.is_some() && self.expr_1()?) {
self.eval_trigger_1_0()?;
}
if (internalevent.y_drift.is_some() && self.expr_2()?) {
self.eval_trigger_2_0()?;
}
if (internalevent.y_drift.is_some() && self.expr_3()?) {
self.eval_trigger_3_0()?;
}
if (internalevent.z_drift.is_some() && self.expr_4()?) {
self.eval_trigger_4_0()?;
}
if (internalevent.z_drift.is_some() && self.expr_5()?) {
self.eval_trigger_5_0()?;
}
if (internalevent.multi_ranger_x_drift.is_some() && self.expr_6()?) {
self.eval_trigger_6_0()?;
}
if (internalevent.multi_ranger_x_drift.is_some() && self.expr_7()?) {
self.eval_trigger_7_0()?;
}
if (internalevent.multi_ranger_y_drift.is_some() && self.expr_8()?) {
self.eval_trigger_8_0()?;
}
if (internalevent.multi_ranger_y_drift.is_some() && self.expr_9()?) {
self.eval_trigger_9_0()?;
}
if (internalevent.multi_ranger_z_drift.is_some() && self.expr_10()?) {
self.eval_trigger_10_0()?;
}
if (internalevent.multi_ranger_z_drift.is_some() && self.expr_11()?) {
self.eval_trigger_11_0()?;
}
if (internalevent.pitch.is_some() && self.expr_12()?) {
self.streammemory.trigger_12.shift();
}
if (internalevent.roll.is_some() && self.expr_13()?) {
self.streammemory.trigger_13.shift();
}
if (internalevent.yaw.is_some() && self.expr_14()?) {
self.streammemory.trigger_14.shift();
}
if ((((((internalevent.x.is_some() && internalevent.y.is_some()) && internalevent.z.is_some()) && internalevent.waypoint_x.is_some()) && internalevent.waypoint_y.is_some()) && internalevent.waypoint_z.is_some()) && self.expr_15()?) {
self.streammemory.trigger_15.shift();
}
if (internalevent.static1000 && self.expr_16()?) {
self.streammemory.trigger_16.shift();
}
if (internalevent.static1000 && self.expr_17()?) {
self.streammemory.trigger_17.shift();
}
if (internalevent.pitch.is_some() && self.expr_12()?) {
self.eval_trigger_12_0()?;
}
if (internalevent.roll.is_some() && self.expr_13()?) {
self.eval_trigger_13_0()?;
}
if (internalevent.yaw.is_some() && self.expr_14()?) {
self.eval_trigger_14_0()?;
}
if ((((((internalevent.x.is_some() && internalevent.y.is_some()) && internalevent.z.is_some()) && internalevent.waypoint_x.is_some()) && internalevent.waypoint_y.is_some()) && internalevent.waypoint_z.is_some()) && self.expr_15()?) {
self.eval_trigger_15_0()?;
}
if (internalevent.static1000 && self.expr_16()?) {
self.eval_trigger_16_0()?;
}
if (internalevent.static1000 && self.expr_17()?) {
self.eval_trigger_17_0()?;
}
let verdict = Verdict::new(self)?;
self.clear()?;
self.clear_activations();
Ok(verdict)
}
}
impl Monitor {
pub(crate) fn x_drift(&self) -> Result<f64, MonitorError> {
Ok(self.streammemory.x_drift.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn y_drift(&self) -> Result<f64, MonitorError> {
Ok(self.streammemory.y_drift.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn z_drift(&self) -> Result<f64, MonitorError> {
Ok(self.streammemory.z_drift.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn pitch(&self) -> Result<f64, MonitorError> {
Ok(self.streammemory.pitch.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn roll(&self) -> Result<f64, MonitorError> {
Ok(self.streammemory.roll.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn yaw(&self) -> Result<f64, MonitorError> {
Ok(self.streammemory.yaw.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn multi_ranger_x_drift(&self) -> Result<f64, MonitorError> {
Ok(self.streammemory.multi_ranger_x_drift.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn multi_ranger_y_drift(&self) -> Result<f64, MonitorError> {
Ok(self.streammemory.multi_ranger_y_drift.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn multi_ranger_z_drift(&self) -> Result<f64, MonitorError> {
Ok(self.streammemory.multi_ranger_z_drift.get(0)?.cloned().expect("sync access"))
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
pub(crate) fn waypoint_x(&self) -> Result<f64, MonitorError> {
Ok(self.streammemory.waypoint_x.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn waypoint_y(&self) -> Result<f64, MonitorError> {
Ok(self.streammemory.waypoint_y.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn waypoint_z(&self) -> Result<f64, MonitorError> {
Ok(self.streammemory.waypoint_z.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn abs_pitch(&self) -> Result<f64, MonitorError> {
Ok(self.streammemory.abs_pitch.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn abs_roll(&self) -> Result<f64, MonitorError> {
Ok(self.streammemory.abs_roll.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn abs_yaw(&self) -> Result<f64, MonitorError> {
Ok(self.streammemory.abs_yaw.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn x_drift_pos_exceeded(&self) -> Result<bool, MonitorError> {
Ok(self.streammemory.x_drift_pos_exceeded.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn x_drift_neg_exceeded(&self) -> Result<bool, MonitorError> {
Ok(self.streammemory.x_drift_neg_exceeded.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn y_drift_pos_exceeded(&self) -> Result<bool, MonitorError> {
Ok(self.streammemory.y_drift_pos_exceeded.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn y_drift_neg_exceeded(&self) -> Result<bool, MonitorError> {
Ok(self.streammemory.y_drift_neg_exceeded.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn z_drift_pos_exceeded(&self) -> Result<bool, MonitorError> {
Ok(self.streammemory.z_drift_pos_exceeded.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn z_drift_neg_exceeded(&self) -> Result<bool, MonitorError> {
Ok(self.streammemory.z_drift_neg_exceeded.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn multi_ranger_x_drift_pos_exceeded(&self) -> Result<bool, MonitorError> {
Ok(self.streammemory.multi_ranger_x_drift_pos_exceeded.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn multi_ranger_x_drift_neg_exceeded(&self) -> Result<bool, MonitorError> {
Ok(self.streammemory.multi_ranger_x_drift_neg_exceeded.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn multi_ranger_y_drift_pos_exceeded(&self) -> Result<bool, MonitorError> {
Ok(self.streammemory.multi_ranger_y_drift_pos_exceeded.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn multi_ranger_y_drift_neg_exceeded(&self) -> Result<bool, MonitorError> {
Ok(self.streammemory.multi_ranger_y_drift_neg_exceeded.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn multi_ranger_z_drift_pos_exceeded(&self) -> Result<bool, MonitorError> {
Ok(self.streammemory.multi_ranger_z_drift_pos_exceeded.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn multi_ranger_z_drift_neg_exceeded(&self) -> Result<bool, MonitorError> {
Ok(self.streammemory.multi_ranger_z_drift_neg_exceeded.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn pitch_exceeded(&self) -> Result<bool, MonitorError> {
Ok(self.streammemory.pitch_exceeded.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn roll_exceeded(&self) -> Result<bool, MonitorError> {
Ok(self.streammemory.roll_exceeded.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn yaw_exceeded(&self) -> Result<bool, MonitorError> {
Ok(self.streammemory.yaw_exceeded.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn dist_to_waypoint(&self) -> Result<f64, MonitorError> {
Ok(self.streammemory.dist_to_waypoint.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn prev_dist_to_waypoint(&self) -> Result<f64, MonitorError> {
Ok(self.streammemory.prev_dist_to_waypoint.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn reached_waypoint(&self) -> Result<bool, MonitorError> {
Ok(self.streammemory.reached_waypoint.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn min_closure_2s(&self) -> Result<f64, MonitorError> {
Ok(self.streammemory.min_closure_2s.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn progressing_2s(&self) -> Result<bool, MonitorError> {
Ok(self.streammemory.progressing_2s.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn min_closure_5s(&self) -> Result<f64, MonitorError> {
Ok(self.streammemory.min_closure_5s.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn progressing_5s(&self) -> Result<bool, MonitorError> {
Ok(self.streammemory.progressing_5s.get(0)?.cloned().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn dist_to_waypoint_offset(&self,offset: usize) -> Result<Option<f64>, MonitorError> {
Ok(self.streammemory.dist_to_waypoint.get(offset)?.cloned())
}
}
impl Monitor {
pub(crate) fn reached_waypoint_hold(&self) -> Result<Option<bool>, MonitorError> {
Ok(self.streammemory.reached_waypoint.get(0)?.cloned())
}
}
impl Monitor {
pub(crate) fn abs_pitch_get(&self) -> Result<Option<f64>, MonitorError> {
Ok(if self.streammemory.abs_pitch.is_fresh() {
self.streammemory.abs_pitch.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn abs_roll_get(&self) -> Result<Option<f64>, MonitorError> {
Ok(if self.streammemory.abs_roll.is_fresh() {
self.streammemory.abs_roll.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn abs_yaw_get(&self) -> Result<Option<f64>, MonitorError> {
Ok(if self.streammemory.abs_yaw.is_fresh() {
self.streammemory.abs_yaw.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn x_drift_pos_exceeded_get(&self) -> Result<Option<bool>, MonitorError> {
Ok(if self.streammemory.x_drift_pos_exceeded.is_fresh() {
self.streammemory.x_drift_pos_exceeded.get(0)?.cloned()
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
pub(crate) fn x_drift_neg_exceeded_get(&self) -> Result<Option<bool>, MonitorError> {
Ok(if self.streammemory.x_drift_neg_exceeded.is_fresh() {
self.streammemory.x_drift_neg_exceeded.get(0)?.cloned()
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
pub(crate) fn y_drift_pos_exceeded_get(&self) -> Result<Option<bool>, MonitorError> {
Ok(if self.streammemory.y_drift_pos_exceeded.is_fresh() {
self.streammemory.y_drift_pos_exceeded.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn trigger_2_get(&self) -> Result<Option<String>, MonitorError> {
Ok(if self.streammemory.trigger_2.is_fresh() {
self.streammemory.trigger_2.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn y_drift_neg_exceeded_get(&self) -> Result<Option<bool>, MonitorError> {
Ok(if self.streammemory.y_drift_neg_exceeded.is_fresh() {
self.streammemory.y_drift_neg_exceeded.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn trigger_3_get(&self) -> Result<Option<String>, MonitorError> {
Ok(if self.streammemory.trigger_3.is_fresh() {
self.streammemory.trigger_3.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn z_drift_pos_exceeded_get(&self) -> Result<Option<bool>, MonitorError> {
Ok(if self.streammemory.z_drift_pos_exceeded.is_fresh() {
self.streammemory.z_drift_pos_exceeded.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn trigger_4_get(&self) -> Result<Option<String>, MonitorError> {
Ok(if self.streammemory.trigger_4.is_fresh() {
self.streammemory.trigger_4.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn z_drift_neg_exceeded_get(&self) -> Result<Option<bool>, MonitorError> {
Ok(if self.streammemory.z_drift_neg_exceeded.is_fresh() {
self.streammemory.z_drift_neg_exceeded.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn trigger_5_get(&self) -> Result<Option<String>, MonitorError> {
Ok(if self.streammemory.trigger_5.is_fresh() {
self.streammemory.trigger_5.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn multi_ranger_x_drift_pos_exceeded_get(&self) -> Result<Option<bool>, MonitorError> {
Ok(if self.streammemory.multi_ranger_x_drift_pos_exceeded.is_fresh() {
self.streammemory.multi_ranger_x_drift_pos_exceeded.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn trigger_6_get(&self) -> Result<Option<String>, MonitorError> {
Ok(if self.streammemory.trigger_6.is_fresh() {
self.streammemory.trigger_6.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn multi_ranger_x_drift_neg_exceeded_get(&self) -> Result<Option<bool>, MonitorError> {
Ok(if self.streammemory.multi_ranger_x_drift_neg_exceeded.is_fresh() {
self.streammemory.multi_ranger_x_drift_neg_exceeded.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn trigger_7_get(&self) -> Result<Option<String>, MonitorError> {
Ok(if self.streammemory.trigger_7.is_fresh() {
self.streammemory.trigger_7.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn multi_ranger_y_drift_pos_exceeded_get(&self) -> Result<Option<bool>, MonitorError> {
Ok(if self.streammemory.multi_ranger_y_drift_pos_exceeded.is_fresh() {
self.streammemory.multi_ranger_y_drift_pos_exceeded.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn trigger_8_get(&self) -> Result<Option<String>, MonitorError> {
Ok(if self.streammemory.trigger_8.is_fresh() {
self.streammemory.trigger_8.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn multi_ranger_y_drift_neg_exceeded_get(&self) -> Result<Option<bool>, MonitorError> {
Ok(if self.streammemory.multi_ranger_y_drift_neg_exceeded.is_fresh() {
self.streammemory.multi_ranger_y_drift_neg_exceeded.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn trigger_9_get(&self) -> Result<Option<String>, MonitorError> {
Ok(if self.streammemory.trigger_9.is_fresh() {
self.streammemory.trigger_9.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn multi_ranger_z_drift_pos_exceeded_get(&self) -> Result<Option<bool>, MonitorError> {
Ok(if self.streammemory.multi_ranger_z_drift_pos_exceeded.is_fresh() {
self.streammemory.multi_ranger_z_drift_pos_exceeded.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn trigger_10_get(&self) -> Result<Option<String>, MonitorError> {
Ok(if self.streammemory.trigger_10.is_fresh() {
self.streammemory.trigger_10.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn multi_ranger_z_drift_neg_exceeded_get(&self) -> Result<Option<bool>, MonitorError> {
Ok(if self.streammemory.multi_ranger_z_drift_neg_exceeded.is_fresh() {
self.streammemory.multi_ranger_z_drift_neg_exceeded.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn trigger_11_get(&self) -> Result<Option<String>, MonitorError> {
Ok(if self.streammemory.trigger_11.is_fresh() {
self.streammemory.trigger_11.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn pitch_exceeded_get(&self) -> Result<Option<bool>, MonitorError> {
Ok(if self.streammemory.pitch_exceeded.is_fresh() {
self.streammemory.pitch_exceeded.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn trigger_12_get(&self) -> Result<Option<String>, MonitorError> {
Ok(if self.streammemory.trigger_12.is_fresh() {
self.streammemory.trigger_12.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn roll_exceeded_get(&self) -> Result<Option<bool>, MonitorError> {
Ok(if self.streammemory.roll_exceeded.is_fresh() {
self.streammemory.roll_exceeded.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn trigger_13_get(&self) -> Result<Option<String>, MonitorError> {
Ok(if self.streammemory.trigger_13.is_fresh() {
self.streammemory.trigger_13.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn yaw_exceeded_get(&self) -> Result<Option<bool>, MonitorError> {
Ok(if self.streammemory.yaw_exceeded.is_fresh() {
self.streammemory.yaw_exceeded.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn trigger_14_get(&self) -> Result<Option<String>, MonitorError> {
Ok(if self.streammemory.trigger_14.is_fresh() {
self.streammemory.trigger_14.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn dist_to_waypoint_get(&self) -> Result<Option<f64>, MonitorError> {
Ok(if self.streammemory.dist_to_waypoint.is_fresh() {
self.streammemory.dist_to_waypoint.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn prev_dist_to_waypoint_get(&self) -> Result<Option<f64>, MonitorError> {
Ok(if self.streammemory.prev_dist_to_waypoint.is_fresh() {
self.streammemory.prev_dist_to_waypoint.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn step_closure_get(&self) -> Result<Option<f64>, MonitorError> {
Ok(if self.streammemory.step_closure.is_fresh() {
self.streammemory.step_closure.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn reached_waypoint_get(&self) -> Result<Option<bool>, MonitorError> {
Ok(if self.streammemory.reached_waypoint.is_fresh() {
self.streammemory.reached_waypoint.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn min_closure_2s_get(&self) -> Result<Option<f64>, MonitorError> {
Ok(if self.streammemory.min_closure_2s.is_fresh() {
self.streammemory.min_closure_2s.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn progressing_2s_get(&self) -> Result<Option<bool>, MonitorError> {
Ok(if self.streammemory.progressing_2s.is_fresh() {
self.streammemory.progressing_2s.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn min_closure_5s_get(&self) -> Result<Option<f64>, MonitorError> {
Ok(if self.streammemory.min_closure_5s.is_fresh() {
self.streammemory.min_closure_5s.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn progressing_5s_get(&self) -> Result<Option<bool>, MonitorError> {
Ok(if self.streammemory.progressing_5s.is_fresh() {
self.streammemory.progressing_5s.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn trigger_15_get(&self) -> Result<Option<String>, MonitorError> {
Ok(if self.streammemory.trigger_15.is_fresh() {
self.streammemory.trigger_15.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn trigger_16_get(&self) -> Result<Option<String>, MonitorError> {
Ok(if self.streammemory.trigger_16.is_fresh() {
self.streammemory.trigger_16.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn trigger_17_get(&self) -> Result<Option<String>, MonitorError> {
Ok(if self.streammemory.trigger_17.is_fresh() {
self.streammemory.trigger_17.get(0)?.cloned()
} else { None })
}
}
impl Monitor {
pub(crate) fn eval_x_drift(&mut self,new_value: f64) -> Result<(), MonitorError> {

self.streammemory.x_drift.update(new_value)
}
}
impl Monitor {
pub(crate) fn eval_y_drift(&mut self,new_value: f64) -> Result<(), MonitorError> {

self.streammemory.y_drift.update(new_value)
}
}
impl Monitor {
pub(crate) fn eval_z_drift(&mut self,new_value: f64) -> Result<(), MonitorError> {

self.streammemory.z_drift.update(new_value)
}
}
impl Monitor {
pub(crate) fn eval_pitch(&mut self,new_value: f64) -> Result<(), MonitorError> {

self.streammemory.pitch.update(new_value)
}
}
impl Monitor {
pub(crate) fn eval_roll(&mut self,new_value: f64) -> Result<(), MonitorError> {

self.streammemory.roll.update(new_value)
}
}
impl Monitor {
pub(crate) fn eval_yaw(&mut self,new_value: f64) -> Result<(), MonitorError> {

self.streammemory.yaw.update(new_value)
}
}
impl Monitor {
pub(crate) fn eval_multi_ranger_x_drift(&mut self,new_value: f64) -> Result<(), MonitorError> {

self.streammemory.multi_ranger_x_drift.update(new_value)
}
}
impl Monitor {
pub(crate) fn eval_multi_ranger_y_drift(&mut self,new_value: f64) -> Result<(), MonitorError> {

self.streammemory.multi_ranger_y_drift.update(new_value)
}
}
impl Monitor {
pub(crate) fn eval_multi_ranger_z_drift(&mut self,new_value: f64) -> Result<(), MonitorError> {

self.streammemory.multi_ranger_z_drift.update(new_value)
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
pub(crate) fn eval_waypoint_x(&mut self,new_value: f64) -> Result<(), MonitorError> {

self.streammemory.waypoint_x.update(new_value)
}
}
impl Monitor {
pub(crate) fn eval_waypoint_y(&mut self,new_value: f64) -> Result<(), MonitorError> {

self.streammemory.waypoint_y.update(new_value)
}
}
impl Monitor {
pub(crate) fn eval_waypoint_z(&mut self,new_value: f64) -> Result<(), MonitorError> {

self.streammemory.waypoint_z.update(new_value)
}
}
impl Monitor {
pub(crate) fn eval_abs_pitch_0(&mut self) -> Result<(), MonitorError> {
let new_value = (if (self.pitch()? < 0f64) { (-self.pitch()?) } else { self.pitch()? });
;
self.streammemory.abs_pitch.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_abs_roll_0(&mut self) -> Result<(), MonitorError> {
let new_value = (if (self.roll()? < 0f64) { (-self.roll()?) } else { self.roll()? });
;
self.streammemory.abs_roll.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_abs_yaw_0(&mut self) -> Result<(), MonitorError> {
let new_value = (if (self.yaw()? < 0f64) { (-self.yaw()?) } else { self.yaw()? });
;
self.streammemory.abs_yaw.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_x_drift_pos_exceeded_0(&mut self) -> Result<(), MonitorError> {
let new_value = (self.x_drift()? > 0.2f64);
;
self.streammemory.x_drift_pos_exceeded.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_trigger_0_0(&mut self) -> Result<(), MonitorError> {
let new_value = "X drift right (State Estimate) > 0.2m — correcting left!".to_owned();
;
self.streammemory.trigger_0.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_x_drift_neg_exceeded_0(&mut self) -> Result<(), MonitorError> {
let new_value = (self.x_drift()? < -0.2f64);
;
self.streammemory.x_drift_neg_exceeded.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_trigger_1_0(&mut self) -> Result<(), MonitorError> {
let new_value = "X drift left (State Estimate) < -0.2m — correcting right!".to_owned();
;
self.streammemory.trigger_1.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_y_drift_pos_exceeded_0(&mut self) -> Result<(), MonitorError> {
let new_value = (self.y_drift()? > 0.2f64);
;
self.streammemory.y_drift_pos_exceeded.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_trigger_2_0(&mut self) -> Result<(), MonitorError> {
let new_value = "Y drift forward (State Estimate) > 0.2m — correcting back!".to_owned();
;
self.streammemory.trigger_2.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_y_drift_neg_exceeded_0(&mut self) -> Result<(), MonitorError> {
let new_value = (self.y_drift()? < -0.2f64);
;
self.streammemory.y_drift_neg_exceeded.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_trigger_3_0(&mut self) -> Result<(), MonitorError> {
let new_value = "Y drift back (State Estimate) < -0.2m — correcting forward!".to_owned();
;
self.streammemory.trigger_3.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_z_drift_pos_exceeded_0(&mut self) -> Result<(), MonitorError> {
let new_value = (self.z_drift()? > 0.2f64);
;
self.streammemory.z_drift_pos_exceeded.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_trigger_4_0(&mut self) -> Result<(), MonitorError> {
let new_value = "Z drift up (State Estimate) > 0.2m — correcting down!".to_owned();
;
self.streammemory.trigger_4.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_z_drift_neg_exceeded_0(&mut self) -> Result<(), MonitorError> {
let new_value = (self.z_drift()? < -0.2f64);
;
self.streammemory.z_drift_neg_exceeded.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_trigger_5_0(&mut self) -> Result<(), MonitorError> {
let new_value = "Z drift down (State Estimate) < -0.2m — correcting up!".to_owned();
;
self.streammemory.trigger_5.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_multi_ranger_x_drift_pos_exceeded_0(&mut self) -> Result<(), MonitorError> {
let new_value = (self.multi_ranger_x_drift()? > 0.2f64);
;
self.streammemory.multi_ranger_x_drift_pos_exceeded.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_trigger_6_0(&mut self) -> Result<(), MonitorError> {
let new_value = "X drift right (Multi-Ranger) > 0.2m — correcting left!".to_owned();
;
self.streammemory.trigger_6.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_multi_ranger_x_drift_neg_exceeded_0(&mut self) -> Result<(), MonitorError> {
let new_value = (self.multi_ranger_x_drift()? < -0.2f64);
;
self.streammemory.multi_ranger_x_drift_neg_exceeded.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_trigger_7_0(&mut self) -> Result<(), MonitorError> {
let new_value = "X drift left (Multi-Ranger) < -0.2m — correcting right!".to_owned();
;
self.streammemory.trigger_7.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_multi_ranger_y_drift_pos_exceeded_0(&mut self) -> Result<(), MonitorError> {
let new_value = (self.multi_ranger_y_drift()? > 0.2f64);
;
self.streammemory.multi_ranger_y_drift_pos_exceeded.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_trigger_8_0(&mut self) -> Result<(), MonitorError> {
let new_value = "Y drift forward (Multi-Ranger) > 0.2m — correcting back!".to_owned();
;
self.streammemory.trigger_8.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_multi_ranger_y_drift_neg_exceeded_0(&mut self) -> Result<(), MonitorError> {
let new_value = (self.multi_ranger_y_drift()? < -0.2f64);
;
self.streammemory.multi_ranger_y_drift_neg_exceeded.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_trigger_9_0(&mut self) -> Result<(), MonitorError> {
let new_value = "Y drift back (Multi-Ranger) < -0.2m — correcting forward!".to_owned();
;
self.streammemory.trigger_9.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_multi_ranger_z_drift_pos_exceeded_0(&mut self) -> Result<(), MonitorError> {
let new_value = (self.multi_ranger_z_drift()? > 0.2f64);
;
self.streammemory.multi_ranger_z_drift_pos_exceeded.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_trigger_10_0(&mut self) -> Result<(), MonitorError> {
let new_value = "Z drift up (Multi-Ranger) > 0.2m — correcting down!".to_owned();
;
self.streammemory.trigger_10.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_multi_ranger_z_drift_neg_exceeded_0(&mut self) -> Result<(), MonitorError> {
let new_value = (self.multi_ranger_z_drift()? < -0.2f64);
;
self.streammemory.multi_ranger_z_drift_neg_exceeded.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_trigger_11_0(&mut self) -> Result<(), MonitorError> {
let new_value = "Z drift down (Multi-Ranger) < -0.2m — correcting up!".to_owned();
;
self.streammemory.trigger_11.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_pitch_exceeded_0(&mut self) -> Result<(), MonitorError> {
let new_value = (self.abs_pitch()? > 0.3f64);
;
self.streammemory.pitch_exceeded.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_trigger_12_0(&mut self) -> Result<(), MonitorError> {
let new_value = "Pitch exceeded 0.3 radians! Stabilizing!".to_owned();
;
self.streammemory.trigger_12.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_roll_exceeded_0(&mut self) -> Result<(), MonitorError> {
let new_value = (self.abs_roll()? > 0.3f64);
;
self.streammemory.roll_exceeded.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_trigger_13_0(&mut self) -> Result<(), MonitorError> {
let new_value = "Roll exceeded 0.3 radians! Stabilizing!".to_owned();
;
self.streammemory.trigger_13.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_yaw_exceeded_0(&mut self) -> Result<(), MonitorError> {
let new_value = (self.abs_yaw()? > 0.3f64);
;
self.streammemory.yaw_exceeded.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_trigger_14_0(&mut self) -> Result<(), MonitorError> {
let new_value = "Yaw exceeded 0.3 radians! Stabilizing!".to_owned();
;
self.streammemory.trigger_14.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_dist_to_waypoint_0(&mut self) -> Result<(), MonitorError> {
let new_value = (((((self.x()? - self.waypoint_x()?) * (self.x()? - self.waypoint_x()?)) + ((self.y()? - self.waypoint_y()?) * (self.y()? - self.waypoint_y()?))) + ((self.z()? - self.waypoint_z()?) * (self.z()? - self.waypoint_z()?)))).sqrt();
;
self.streammemory.dist_to_waypoint.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_prev_dist_to_waypoint_0(&mut self) -> Result<(), MonitorError> {
let new_value = self.dist_to_waypoint_offset(1)?.unwrap_or(10000f64);
;
self.streammemory.prev_dist_to_waypoint.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_step_closure_0(&mut self) -> Result<(), MonitorError> {
let new_value = (self.prev_dist_to_waypoint()? - self.dist_to_waypoint()?);
self.windowsmemory.sliding0.accept_value(self.time, new_value);
self.windowsmemory.sliding1.accept_value(self.time, new_value);;
self.streammemory.step_closure.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_reached_waypoint_0(&mut self) -> Result<(), MonitorError> {
let new_value = (self.dist_to_waypoint()? <= 0.1f64);
;
self.streammemory.reached_waypoint.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_min_closure_2s_0(&mut self) -> Result<(), MonitorError> {
let new_value = self.windowsmemory.sliding0.get_value(self.time).flatten().unwrap_or_else(||0f64);
;
self.streammemory.min_closure_2s.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_progressing_2s_0(&mut self) -> Result<(), MonitorError> {
let new_value = (self.min_closure_2s()? > 0.02f64);
;
self.streammemory.progressing_2s.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_min_closure_5s_0(&mut self) -> Result<(), MonitorError> {
let new_value = self.windowsmemory.sliding1.get_value(self.time).flatten().unwrap_or_else(||0f64);
;
self.streammemory.min_closure_5s.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_progressing_5s_0(&mut self) -> Result<(), MonitorError> {
let new_value = (self.min_closure_5s()? > 0.02f64);
;
self.streammemory.progressing_5s.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_trigger_15_0(&mut self) -> Result<(), MonitorError> {
let new_value = "Reached waypoint.".to_owned();
;
self.streammemory.trigger_15.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_trigger_16_0(&mut self) -> Result<(), MonitorError> {
let new_value = "Not progressing toward waypoint for 2s.".to_owned();
;
self.streammemory.trigger_16.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_trigger_17_0(&mut self) -> Result<(), MonitorError> {
let new_value = "Potentially stuck: no net approach for 5s.".to_owned();
;
self.streammemory.trigger_17.update(new_value)?;
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