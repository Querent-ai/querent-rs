use crate::callbacks::types::{EventType, EventTypeWrapper};
use pyo3::prelude::*;

#[derive(Debug)]
pub struct EventState {
	pub event_type: EventType,
	pub timestamp: f64,
	pub payload: String,
}

impl From<EventState> for (EventType, f64, String) {
	fn from(event_state: EventState) -> Self {
		(event_state.event_type, event_state.timestamp, event_state.payload)
	}
}

impl Into<EventState> for (EventType, f64, String) {
	fn into(self) -> EventState {
		EventState { event_type: self.0, timestamp: self.1, payload: self.2 }
	}
}

#[pyclass]
#[derive(Debug)]
pub struct EventStateWrapper {
	pub event_state: EventState,
}

#[pymethods]
impl EventStateWrapper {
	#[new]
	pub fn new(event_type: &str, timestamp: f64, payload: &str) -> Self {
		let event_type = EventTypeWrapper::new(event_type).event_type;
		let event_state = EventState { event_type, timestamp, payload: payload.to_string() };
		Self { event_state }
	}
}
