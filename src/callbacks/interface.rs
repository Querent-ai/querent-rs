// Base interface for callbacks
use crate::callbacks::types::event::{EventState, EventType};
use pyo3::prelude::*;

pub trait EventCallbackInterface {
	fn handle_event(&mut self, event_type: EventType, event_data: EventState);
}

#[pyclass]
pub struct PyEventCallbackInterface {}

#[pymethods]
impl PyEventCallbackInterface {
	#[new]
	fn new() -> Self {
		PyEventCallbackInterface {}
	}

	fn handle_event(&mut self, event_type: EventType, event_data: EventState) {
		println!("Event: {:?}, {:?}", event_type, event_data);
		println!("TODO: handle different event types")
	}
}
