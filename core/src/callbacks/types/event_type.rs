use pyo3::prelude::*;

// Define a Rust enum for EventType
#[derive(Debug)]
pub enum EventType {
	StateTransition,
	RdfGraphUpdate,
	TokenProcessed,
	ChatCompleted,
}

#[pyclass]
pub struct EventTypeWrapper {
	pub event_type: EventType,
}

#[pymethods]
impl EventTypeWrapper {
	#[new]
	pub fn new(event_type: &str) -> Self {
		let event_type = match event_type {
			"state_transition" => EventType::StateTransition,
			"rdf_graph_update" => EventType::RdfGraphUpdate,
			"token_processed" => EventType::TokenProcessed,
			"chat_completed" => EventType::ChatCompleted,
			_ => {
				// Handle unknown event types or raise an error
				// You can define your error handling logic here
				EventType::StateTransition // Use a default value if needed
			},
		};
		Self { event_type }
	}
}
