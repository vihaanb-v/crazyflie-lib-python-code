use core::ptr;
use std::fmt::Display;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::process::exit;
use std::time::Duration;
#[derive(Debug, Clone)]
			enum MonitorError {
			InstanceNotFound { stream: &'static str, instance: String },
			OutOfBoundsAccess { accessed_offset: usize, buffer_size: usize }
		}
pub struct Monitor {
streammemory: StreamMemory,
time: std::time::Duration,
}
impl Monitor {
pub fn new(start_time: std::time::Duration) -> Self {
Self {
            streammemory: StreamMemory::new(start_time),
time: start_time
	}
}
}
struct StreamMemory {
a: StreamBuffer<i64, 1>,
b: StreamBuffer<i64, 1>,
c: DynamicStreamBuffer<i64, 1>,
d: DynamicStreamBuffer<i64, 1>,
e: DynamicStreamBuffer<i64, 1>,
f: DynamicStreamBuffer<i64, 1>,
g: DynamicStreamBuffer<i64, 1>,
}
impl StreamMemory {
fn new(start_time: std::time::Duration) -> Self {
Self {
a: StreamBuffer::new(),
b: StreamBuffer::new(),
c: DynamicStreamBuffer::new(true),
d: DynamicStreamBuffer::new(true),
e: DynamicStreamBuffer::new(true),
f: DynamicStreamBuffer::new(true),
g: DynamicStreamBuffer::new(true)
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
        let values = std::array::from_fn(|_| None);
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

#[derive(Clone, Debug)]
pub(crate) struct DynamicStreamBuffer<StreamType, const STREAM_SIZE: usize> {
    stream_buffer: StreamBuffer<StreamType, STREAM_SIZE>,
    alive: bool,
}

impl<StreamType, const STREAM_SIZE: usize> DynamicStreamBuffer<StreamType, STREAM_SIZE> {
    pub(crate) fn new(alive: bool) -> Self {
        Self {
            stream_buffer: StreamBuffer::new(),
            alive,
        }
    }
}

impl<StreamType, const STREAM_SIZE: usize> StreamBufferTrait<StreamType, STREAM_SIZE>
    for DynamicStreamBuffer<StreamType, STREAM_SIZE>
{
    fn stream_buffer(&self) -> &StreamBuffer<StreamType, STREAM_SIZE> {
        &self.stream_buffer
    }

    fn stream_buffer_as_mut(&mut self) -> &mut StreamBuffer<StreamType, STREAM_SIZE> {
        &mut self.stream_buffer
    }
}

impl<StreamType, const STREAM_SIZE: usize> DynamicStreamBuffer<StreamType, STREAM_SIZE> {
    pub(crate) fn is_alive(&self) -> bool {
        self.alive
    }

    pub(crate) fn spawn(&mut self) -> Result<(), MonitorError> {
        self.alive = true;
        Ok(())
    }

    pub(crate) fn close(&mut self) -> Result<(), MonitorError> {
        self.alive = false;
        self.stream_buffer = StreamBuffer::new();
        Ok(())
    }
}

enum StreamReference {
A,
B,
C,
D,
E,
F,
G,
}
#[derive(Debug, Clone, PartialEq, Eq)]
enum Deadline {

}
pub struct Event {
b: Option<i64>,
a: Option<i64>,
}
struct InternalEvent {
a: Option<i64>,
b: Option<i64>,
time: std::time::Duration,
}
impl InternalEvent {
fn from_event(time: std::time::Duration, event: Event) -> InternalEvent {
Self {
a: event.a
,b: event.b
,time: time
}
}
}
pub struct Verdict {
f: Option<i64>,
g: Option<i64>,
e: Option<i64>,
c: Option<i64>,
d: Option<i64>,
time: std::time::Duration,
}
impl Verdict {
fn new(monitor: &mut Monitor) -> Result<Verdict, MonitorError> {
Ok(Self {
c: monitor.c_get()?,
d: monitor.d_get()?,
e: monitor.e_get()?,
f: monitor.f_get()?,
g: monitor.g_get()?,
time: monitor.time
})
}
}

impl Display for Verdict {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},", self.c.as_ref().map(|v|v.to_string()).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.d.as_ref().map(|v|v.to_string()).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.e.as_ref().map(|v|v.to_string()).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.f.as_ref().map(|v|v.to_string()).unwrap_or_else(||"#".into()))?;
write!(f, "{},", self.g.as_ref().map(|v|v.to_string()).unwrap_or_else(||"#".into()))?;
		writeln!(f, "{}", self.time.as_secs_f64())
        }
    }
impl Monitor {
fn clear_activations(&mut self) -> () {
self.streammemory.a.clear_activation();
self.streammemory.b.clear_activation();
self.streammemory.c.clear_activation();
self.streammemory.d.clear_activation();
self.streammemory.e.clear_activation();
self.streammemory.f.clear_activation();
self.streammemory.g.clear_activation();
}
}
impl Monitor {
fn cycle(&mut self,internalevent: InternalEvent) -> Result<Verdict, MonitorError> {
self.time = internalevent.time;
if internalevent.a.is_some() {
self.streammemory.a.shift();
self.eval_a(internalevent.a.expect("Checked the existence with the guard"))?;
}
if internalevent.b.is_some() {
self.streammemory.b.shift();
self.eval_b(internalevent.b.expect("Checked the existence with the guard"))?;
}
if true {
self.streammemory.c.spawn()?;
}
if true {
self.streammemory.d.spawn()?;
}
if true {
self.streammemory.e.spawn()?;
}
if true {
self.streammemory.f.spawn()?;
}
if true {
self.streammemory.g.spawn()?;
}
if self.streammemory.c.is_alive() {
if (internalevent.a.is_some() && internalevent.b.is_some()) {
self.streammemory.c.shift();
}
}
if self.streammemory.d.is_alive() {
if (internalevent.a.is_some() && internalevent.b.is_some()) {
self.streammemory.d.shift();
}
}
if self.streammemory.e.is_alive() {
if (internalevent.a.is_some() && internalevent.b.is_some()) {
self.streammemory.e.shift();
}
}
if self.streammemory.f.is_alive() {
if internalevent.a.is_some() {
self.streammemory.f.shift();
}
}
if self.streammemory.g.is_alive() {
if internalevent.b.is_some() {
self.streammemory.g.shift();
}
}
if self.streammemory.c.is_alive() {
if (internalevent.a.is_some() && internalevent.b.is_some()) {
self.eval_c()?;
}
}
if self.streammemory.d.is_alive() {
if (internalevent.a.is_some() && internalevent.b.is_some()) {
self.eval_d()?;
}
}
if self.streammemory.e.is_alive() {
if (internalevent.a.is_some() && internalevent.b.is_some()) {
self.eval_e()?;
}
}
if self.streammemory.f.is_alive() {
if internalevent.a.is_some() {
self.eval_f()?;
}
}
if self.streammemory.g.is_alive() {
if internalevent.b.is_some() {
self.eval_g()?;
}
}
if self.streammemory.c.is_alive() {
if false {
self.streammemory.c.close()?;
}
}
if self.streammemory.d.is_alive() {
if false {
self.streammemory.d.close()?;
}
}
if self.streammemory.e.is_alive() {
if false {
self.streammemory.e.close()?;
}
}
if self.streammemory.f.is_alive() {
if false {
self.streammemory.f.close()?;
}
}
if self.streammemory.g.is_alive() {
if false {
self.streammemory.g.close()?;
}
}
let verdict = Verdict::new(self)?;
self.clear_activations();
Ok(verdict)
}
}
impl Monitor {
pub(crate) fn a(&self) -> Result<i64, MonitorError> {
Ok(self.streammemory.a.get(0)?.copied().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn b(&self) -> Result<i64, MonitorError> {
Ok(self.streammemory.b.get(0)?.copied().expect("sync access"))
}
}
impl Monitor {
pub(crate) fn c_get(&self) -> Result<Option<i64>, MonitorError> {
Ok(if self.streammemory.c.is_fresh() {
self.streammemory.c.get(0)?.copied()
} else { None })
}
}
impl Monitor {
pub(crate) fn d_get(&self) -> Result<Option<i64>, MonitorError> {
Ok(if self.streammemory.d.is_fresh() {
self.streammemory.d.get(0)?.copied()
} else { None })
}
}
impl Monitor {
pub(crate) fn e_get(&self) -> Result<Option<i64>, MonitorError> {
Ok(if self.streammemory.e.is_fresh() {
self.streammemory.e.get(0)?.copied()
} else { None })
}
}
impl Monitor {
pub(crate) fn f_get(&self) -> Result<Option<i64>, MonitorError> {
Ok(if self.streammemory.f.is_fresh() {
self.streammemory.f.get(0)?.copied()
} else { None })
}
}
impl Monitor {
pub(crate) fn g_get(&self) -> Result<Option<i64>, MonitorError> {
Ok(if self.streammemory.g.is_fresh() {
self.streammemory.g.get(0)?.copied()
} else { None })
}
}
impl Monitor {
pub(crate) fn eval_a(&mut self,new_value: i64) -> Result<(), MonitorError> {
self.streammemory.a.update(new_value)
}
}
impl Monitor {
pub(crate) fn eval_b(&mut self,new_value: i64) -> Result<(), MonitorError> {
self.streammemory.b.update(new_value)
}
}
impl Monitor {
pub(crate) fn eval_c(&mut self) -> Result<(), MonitorError> {
let new_value = (self.a()?+self.b()?);
self.streammemory.c.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_d(&mut self) -> Result<(), MonitorError> {
let new_value = (self.a()?-self.b()?);
self.streammemory.d.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_e(&mut self) -> Result<(), MonitorError> {
let new_value = (self.a()?*self.b()?);
self.streammemory.e.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_f(&mut self) -> Result<(), MonitorError> {
let new_value = (self.a()?+1);
self.streammemory.f.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub(crate) fn eval_g(&mut self) -> Result<(), MonitorError> {
let new_value = (self.b()?-1);
self.streammemory.g.update(new_value)?;
Ok(())
}
}
impl Monitor {
pub fn accept_event(&mut self,event: Event, time: std::time::Duration) -> Result<Vec<Verdict>, MonitorError> {
let event = InternalEvent::from_event(time, event);
let mut verdicts = self.accept_time(time)?;
verdicts.push(self.cycle(event)?);
Ok(verdicts)
}
}
impl Monitor {
pub fn accept_time(&mut self,time: std::time::Duration) -> Result<Vec<Verdict>, MonitorError> {
Ok(Vec::new())
}
}
impl Monitor {
pub fn close(&mut self,time: std::time::Duration) -> Result<Vec<Verdict>, MonitorError> {
Ok(Vec::new())
}
}
impl Event {
    fn from_csv(line: &str) -> (Self, Duration) {
        let mut values = line.split(",");
        let a_str = values.next().expect("expecting value for input a").trim();
        let a = (a_str != "#").then(|| a_str.parse().unwrap());
        let b_str = values.next().expect("expecting value for input b").trim();
        let b = (b_str != "#").then(|| b_str.parse().unwrap());

        let time_str = values.next().expect("expecting value for time").trim();
        let time = Duration::from_secs_f64(time_str.parse::<f64>().expect("error parsing time"));

        (Event { a,b, }, time)
    }
}

fn main() {
    let trace_path = std::env::args().nth(1).expect("Give trace as first argument");
    let trace_file = File::open(trace_path).unwrap();
    let mut csv_lines = BufReader::new(trace_file).lines().skip(1); // skip header file

    println!("c,d,e,f,g,time"); // verdict csv header

    let first_line = csv_lines.next().unwrap_or_else(|| exit(0)).unwrap();
    let (first_event, start_time) = Event::from_csv(&first_line);

    let mut monitor = Monitor::new(start_time);
    let verdicts = monitor
        .accept_event(first_event, start_time)
        .expect("error monitoring event");
    verdicts.iter().for_each(|v| print!("{v}"));

    let mut current_time = Duration::new(0,0);
    for csv_line in csv_lines.map(|line| line.unwrap()) {
        let (event, time) = Event::from_csv(&csv_line);
        let verdicts = monitor.accept_event(event, time).expect("error monitoring event");
        unsafe {
        std::mem::forget(ptr::read_volatile(&verdicts));
        }
        verdicts.iter().for_each(|v| print!("{v}"));
        current_time = time;
    }
    let verdicts = monitor.close(current_time).expect("error closing monitor");
    verdicts.iter().for_each(|v| print!("{v}"));
    unsafe {
    std::mem::forget(ptr::read_volatile(&verdicts));
    }
}