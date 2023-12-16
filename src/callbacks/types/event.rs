// Import necessary items from the pyo3 crate
use pyo3::{exceptions::PyTypeError, prelude::*};

// Define an enumeration for different event types
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EventType {
	ContextualGraphUpdated,
	SemanticGraphUpdated,
	ChatCompleted,
}

// Implement conversion from Python object to EventType
impl<'a> FromPyObject<'a> for EventType {
	fn extract(ob: &'a PyAny) -> PyResult<Self> {
		// Try to extract a string from the Python object
		if let Ok(event_type) = ob.extract::<&str>() {
			// Match the string to determine the EventType
			match event_type {
				"chat_completed" => Ok(EventType::ChatCompleted),
				"contextual_graph_updated" => Ok(EventType::ContextualGraphUpdated),
				"semantic_graph_updated" => Ok(EventType::SemanticGraphUpdated),
				// If the string does not match any known EventType, return an error
				_ => Err(PyErr::new::<PyTypeError, _>("Invalid event type")),
			}
		} else {
			// If extraction fails, return an error
			Err(PyErr::new::<PyTypeError, _>("Invalid event type"))
		}
	}
}

// Define a structure to represent the state of an event
#[derive(Clone, Debug, PartialEq)]
pub struct EventState {
	pub event_type: EventType,
	pub timestamp: f64,
	pub payload: String,
}

// Implement conversion from Python object to EventState
impl<'a> FromPyObject<'a> for EventState {
	fn extract(ob: &'a PyAny) -> PyResult<Self> {
		// Extract values for event_type, timestamp, and payload from the Python object
		let event_type = ob.get_item("event_type")?.extract()?;
		let timestamp = ob.get_item("timestamp")?.extract()?;
		let payload = ob.get_item("payload")?.extract()?;
		// Create and return an EventState instance
		Ok(EventState { event_type, timestamp, payload })
	}
}
