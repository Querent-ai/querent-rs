// Base interface for callbacks
use crate::callbacks::types::event::{EventState, EventType};
use pyo3::prelude::*;

pub trait EventCallbackInterface {
	fn handle_event(&mut self, event_type: EventType, event_data: EventState);
}

#[derive(Clone, Debug)]
pub struct EventHandler {}

impl EventHandler {
	pub fn new() -> Self {
		EventHandler {}
	}
}

#[derive(Clone, Debug)]
#[pyclass]
pub struct PyEventCallbackInterface {
	event_handler: EventHandler,
}

#[pymethods]
impl PyEventCallbackInterface {
	#[new]
	pub fn new() -> Self {
		PyEventCallbackInterface { event_handler: EventHandler::new() }
	}

	fn handle_event(&mut self, event_type: EventType, event_data: EventState) {
		self.event_handler.handle_event(event_type, event_data);
	}
}

impl EventCallbackInterface for EventHandler {
	fn handle_event(&mut self, event_type: EventType, event_data: EventState) {
		println!("Event: {:?}, {:?}", event_type, event_data);
		println!("TODO: handle different event types")
	}
}
