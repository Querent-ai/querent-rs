use crate::callbacks::types::{EventType, EventTypeWrapper};
use pyo3::prelude::*;

#[derive(Debug)]
pub struct EventState {
	pub event_type: EventType,
	pub timestamp: f64,
	pub payload: String,
}

#[pyclass]
pub struct EventStateWrapper {
	pub event_state: EventState,
}

#[pymethods]
impl EventStateWrapper {
	#[new]
	fn new(event_type: &str, timestamp: f64, payload: &str) -> Self {
		let event_type = EventTypeWrapper::new(event_type).event_type;
		let event_state = EventState { event_type, timestamp, payload: payload.to_string() };
		Self { event_state }
	}
}
