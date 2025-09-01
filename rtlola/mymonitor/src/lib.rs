use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;           // PyResult, PyErr, Bound, #[pyclass], etc.
use pyo3::types::PyModule;      // PyModule lives in types::
use std::time::Duration;

// bring the generated monitor as a module, then import its items
#[path = "core.rs"]
mod core;
use crate::core::{Event, Monitor, MonitorError, Verdict};

impl From<MonitorError> for PyErr {
    fn from(e: MonitorError) -> Self {
        PyRuntimeError::new_err(format!("{e:?}"))
    }
}

fn secs(s: f64) -> Duration {
    if s.is_sign_negative() {
        Duration::from_secs(0)
    } else {
        let secs = s.trunc() as u64;
        let nanos = (s.fract() * 1e9).round() as u32;
        Duration::new(secs, nanos)
    }
}

#[pyclass]
#[derive(Clone)]
struct PyEvent {
    #[pyo3(get, set)] z: Option<f64>,
    #[pyo3(get, set)] y_drift: Option<f64>,
    #[pyo3(get, set)] multi_ranger_x_drift: Option<f64>,
    #[pyo3(get, set)] multi_ranger_y_drift: Option<f64>,
    #[pyo3(get, set)] multi_ranger_z_drift: Option<f64>,
    #[pyo3(get, set)] roll: Option<f64>,
    #[pyo3(get, set)] waypoint_z: Option<f64>,
    #[pyo3(get, set)] z_drift: Option<f64>,
    #[pyo3(get, set)] yaw: Option<f64>,
    #[pyo3(get, set)] x: Option<f64>,
    #[pyo3(get, set)] y: Option<f64>,
    #[pyo3(get, set)] pitch: Option<f64>,
    #[pyo3(get, set)] waypoint_y: Option<f64>,
    #[pyo3(get, set)] x_drift: Option<f64>,
    #[pyo3(get, set)] waypoint_x: Option<f64>,
}

#[pymethods]
impl PyEvent {
    #[new]
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (
        z=None, y_drift=None, multi_ranger_x_drift=None, multi_ranger_y_drift=None,
        multi_ranger_z_drift=None, roll=None, waypoint_z=None, z_drift=None, yaw=None,
        x=None, y=None, pitch=None, waypoint_y=None, x_drift=None, waypoint_x=None
    ))]
    fn new(
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
    ) -> Self {
        Self {
            z, y_drift, multi_ranger_x_drift, multi_ranger_y_drift, multi_ranger_z_drift, roll,
            waypoint_z, z_drift, yaw, x, y, pitch, waypoint_y, x_drift, waypoint_x,
        }
    }
}

impl From<PyEvent> for Event {
    fn from(e: PyEvent) -> Self {
        Event {
            z: e.z,
            y_drift: e.y_drift,
            multi_ranger_x_drift: e.multi_ranger_x_drift,
            multi_ranger_y_drift: e.multi_ranger_y_drift,
            multi_ranger_z_drift: e.multi_ranger_z_drift,
            roll: e.roll,
            waypoint_z: e.waypoint_z,
            z_drift: e.z_drift,
            yaw: e.yaw,
            x: e.x,
            y: e.y,
            pitch: e.pitch,
            waypoint_y: e.waypoint_y,
            x_drift: e.x_drift,
            waypoint_x: e.waypoint_x,
        }
    }
}

#[pyclass]
struct PyVerdict {
    inner: Verdict,
}

#[pymethods]
impl PyVerdict {
    fn to_csv(&self) -> String { format!("{}", self.inner) }
    fn __repr__(&self) -> PyResult<String> { Ok(self.to_csv()) }
    fn __str__(&self) -> PyResult<String> { Ok(self.to_csv()) }
}

fn wrap_vec<'py>(py: Python<'py>, vs: Vec<Verdict>) -> PyResult<Vec<Py<PyVerdict>>> {
    vs.into_iter()
        .map(|inner| Py::new(py, PyVerdict { inner }))
        .collect()
}

#[pyclass]
struct PyMonitor {
    inner: Monitor,
}

#[pymethods]
impl PyMonitor {
    #[new]
    #[pyo3(signature = (start_time_secs=None))]
    fn new(start_time_secs: Option<f64>) -> PyResult<Self> {
        Ok(Self { inner: Monitor::new(secs(start_time_secs.unwrap_or(0.0))) })
    }

    /// Accept inputs at timestamp `t_secs` and return verdicts (including any timer events up to t).
    fn accept<'py>(&mut self, py: Python<'py>, event: PyEvent, t_secs: f64) -> PyResult<Vec<Py<PyVerdict>>> {
        let vs = self.inner.accept_event(event.into(), secs(t_secs))?;
        wrap_vec(py, vs)
    }

    /// Advance time without inputs; returns internal verdicts up to `t_secs` (exclusive).
    fn tick<'py>(&mut self, py: Python<'py>, t_secs: f64) -> PyResult<Vec<Py<PyVerdict>>> {
        let vs = self.inner.accept_time(secs(t_secs))?;
        wrap_vec(py, vs)
    }

    /// Close the monitor at `t_secs`; returns trailing verdicts.
    fn close<'py>(&mut self, py: Python<'py>, t_secs: f64) -> PyResult<Vec<Py<PyVerdict>>> {
        let vs = self.inner.close(secs(t_secs))?;
        wrap_vec(py, vs)
    }
}

#[pymodule]
fn mymonitor(m: &Bound<PyModule>) -> PyResult<()> {
    m.add_class::<PyMonitor>()?;
    m.add_class::<PyEvent>()?;
    m.add_class::<PyVerdict>()?;
    Ok(())
}