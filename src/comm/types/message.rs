// Import necessary items from the pyo3 crate
use pyo3::{exceptions::PyTypeError, prelude::*};

// Define an enumeration for different event types
#[derive(Clone, Debug, PartialEq)]
pub enum MessageType {
	Start,
	Stop,
	Pause,
	Resume,
	Restart,
	Status,
	Metrics,
}

// Implement conversion from Python object to EventType
impl<'a> FromPyObject<'a> for MessageType {
	fn extract(ob: &'a PyAny) -> PyResult<Self> {
		// Try to extract a string from the Python object
		if let Ok(message_type) = ob.extract::<&str>() {
			// Match the string to determine the EventType
			match message_type {
				"start" => Ok(MessageType::Start),
				"stop" => Ok(MessageType::Stop),
				"pause" => Ok(MessageType::Pause),
				"resume" => Ok(MessageType::Resume),
				"restart" => Ok(MessageType::Restart),
				"status" => Ok(MessageType::Status),
				"metrics" => Ok(MessageType::Metrics),
				// If the string does not match any known EventType, return an error
				_ => Err(PyErr::new::<PyTypeError, _>("Invalid message type")),
			}
		} else {
			// If extraction fails, return an error
			Err(PyErr::new::<PyTypeError, _>("Invalid message type"))
		}
	}
}

// Define a structure to represent the state of an event
#[derive(Clone, Debug, PartialEq)]
pub struct MessageState {
	pub message_type: MessageType,
	pub timestamp: f64,
	pub payload: String,
}

// Implement conversion from Python object to MessageState
impl<'a> FromPyObject<'a> for MessageState {
	fn extract(ob: &'a PyAny) -> PyResult<Self> {
		// Extract values for event_type, timestamp, and payload from the Python object
		let message_type = ob.get_item("message_type")?.extract()?;
		let timestamp = ob.get_item("timestamp")?.extract()?;
		let payload = ob.get_item("payload")?.extract()?;
		// Create and return an MessageState instance
		Ok(MessageState { message_type, timestamp, payload })
	}
}
