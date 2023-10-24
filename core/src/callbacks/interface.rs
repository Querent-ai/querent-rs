// Base interface for callbacks
use crate::callbacks::types::{EventState, EventType};
use pyo3::prelude::*;

use super::EventStateWrapper;

pub trait EventCallbackInterface {
	fn handle_event(&mut self, event_type: EventType, event_data: EventState);
}

#[pyclass]
pub struct PyEventCallbackInterface {
	callback: Py<PyAny>,
}

#[pymethods]
impl PyEventCallbackInterface {
	#[new]
	fn new(callback: Py<PyAny>) -> Self {
		Self { callback }
	}

	fn handle_event(&mut self, event_type: &str, timestamp: f64, payload: &str) {
		let event_wrapped = EventStateWrapper::new(event_type, timestamp, payload);
		println!("Event: {:?}", event_wrapped);
	}
}
