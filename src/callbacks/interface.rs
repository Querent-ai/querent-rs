// Import necessary items from the callbacks module
use crate::callbacks::types::event::{EventState, EventType};
// Import necessary items from the pyo3 crate
use pyo3::prelude::*;

// Define the base interface for event callbacks
pub trait EventCallbackInterface {
	fn handle_event(&mut self, event_type: EventType, event_data: EventState);
}

// Define a basic event handler struct
#[derive(Clone, Debug)]
pub struct EventHandler {}

impl EventHandler {
	// Constructor for EventHandler
	pub fn new() -> Self {
		EventHandler {}
	}
}

// Define a Python-compatible event callback interface
#[derive(Clone, Debug)]
#[pyclass]
pub struct PyEventCallbackInterface {
	// Internal event handler instance
	event_handler: EventHandler,
}

// Implement Python methods for PyEventCallbackInterface
#[pymethods]
impl PyEventCallbackInterface {
	// Python constructor for PyEventCallbackInterface
	#[new]
	pub fn new() -> Self {
		PyEventCallbackInterface { event_handler: EventHandler::new() }
	}

	// Python method to handle events
	fn handle_event(&mut self, event_type: EventType, event_data: EventState) {
		// Delegate the event handling to the internal event handler
		self.event_handler.handle_event(event_type, event_data);
	}
}

// Implement the EventCallbackInterface for the EventHandler
impl EventCallbackInterface for EventHandler {
	// Implementation of the handle_event method for EventHandler
	fn handle_event(&mut self, event_type: EventType, event_data: EventState) {
		// Print basic information about the event (TODO: Handle different event types)
		println!("Event: {:?}, {:?}", event_type, event_data);
		println!("TODO: handle different event types");
	}
}
