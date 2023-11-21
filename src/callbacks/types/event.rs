use pyo3::{exceptions::PyTypeError, prelude::*};

#[derive(Clone, Debug, PartialEq)]
pub enum EventType {
	ContextualGraphUpdated,
	SemanticGraphUpdated,
	ChatCompleted,
}

impl<'a> FromPyObject<'a> for EventType {
	fn extract(ob: &'a PyAny) -> PyResult<Self> {
		if let Ok(event_type) = ob.extract::<&str>() {
			match event_type {
				"chat_completed" => Ok(EventType::ChatCompleted),
				"contextual_graph_updated" => Ok(EventType::ContextualGraphUpdated),
				"semantic_graph_updated" => Ok(EventType::SemanticGraphUpdated),
				_ => Err(PyErr::new::<PyTypeError, _>("Invalid event type")),
			}
		} else {
			Err(PyErr::new::<PyTypeError, _>("Invalid event type"))
		}
	}
}

#[derive(Clone, Debug, PartialEq)]
pub struct EventState {
	pub event_type: EventType,
	pub timestamp: f64,
	pub payload: String,
}

impl<'a> FromPyObject<'a> for EventState {
	fn extract(ob: &'a PyAny) -> PyResult<Self> {
		let event_type = ob.get_item("event_type")?.extract()?;
		let timestamp = ob.get_item("timestamp")?.extract()?;
		let payload = ob.get_item("payload")?.extract()?;
		Ok(EventState { event_type, timestamp, payload })
	}
}
