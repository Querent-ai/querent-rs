// Import necessary items from the callbacks module
use crate::callbacks::types::event::{EventState, EventType};
// Import necessary items from the pyo3 crate
use pyo3::prelude::*;
use tokio::sync::mpsc;

// Define the base interface for event callbacks
pub trait EventCallbackInterface {
	fn handle_event(&mut self, event_type: EventType, event_data: EventState);
}

// Define a basic event handler struct
#[derive(Clone, Debug)]
#[pyclass]
pub struct EventHandler {
	event_sender: Option<mpsc::Sender<(EventType, EventState)>>,
}

impl EventHandler {
	// Constructor for EventHandler
	pub fn new(event_sender: Option<mpsc::Sender<(EventType, EventState)>>) -> Self {
		EventHandler { event_sender }
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
	pub fn new(event_handler: EventHandler) -> Self {
		PyEventCallbackInterface { event_handler }
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
		// If the event sender is not None, send the event
		if let Some(event_sender) = &self.event_sender {
			// Send the event
			event_sender.try_send((event_type, event_data)).unwrap();
		} else {
			println!("Event sender is None");
			println!("Event type: {:?}", event_type);
			println!("Event data: {:?}", event_data);
		}
	}
}
