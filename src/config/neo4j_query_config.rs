use pyo3::{prelude::*, types::PyDict, PyObject, ToPyObject};

use crate::{
	callbacks::{interface::EventHandler, PyEventCallbackInterface},
	comm::{ChannelHandler, PyMessageInterface},
};

/// Configuration struct representing the overall setup for a system.
#[derive(Debug, Clone)]
#[pyclass]
pub struct Neo4jQueryConfig {
	pub db_name: String,
	pub url: String,
	pub username: String,
	pub password: String,
	pub inner_channel: Option<ChannelHandler>,
	#[pyo3(get, set)]
	pub channel: Option<PyObject>,
	/// Inner EventHandler for workflow to get events from python
	pub inner_event_handler: Option<EventHandler>,
	/// PyObject for the event handler.
	#[pyo3(get, set)]
	pub event_handler: Option<PyObject>,
	/// Token feader for the engine for live tokens
	pub inner_tokens_feader: Option<ChannelHandler>,
	/// Token feeder for the engine for live tokens
	#[pyo3(get, set)]
	pub tokens_feader: Option<PyObject>,
}

impl ToPyObject for Neo4jQueryConfig {
	/// Converts a Neo4jQueryConfig to a Python object.
	fn to_object(&self, py: Python) -> PyObject {
		let neo4j_query_dict = PyDict::new(py);
		neo4j_query_dict.set_item("db_name", &self.db_name).unwrap();
		neo4j_query_dict.set_item("url", &self.url).unwrap();
		neo4j_query_dict.set_item("username", &self.username).unwrap();
		neo4j_query_dict.set_item("password", &self.password).unwrap();
		// convert channel handler to python object
		if let Some(inner_channel) = &self.inner_channel {
			let channel_interface = PyMessageInterface::new(inner_channel.clone());
			let channel: PyObject =
				Py::new(py, channel_interface).expect("Unable to create class").into_py(py);
			neo4j_query_dict.set_item("channel", channel).unwrap();
		}
		// convert event handler to python object
		if let Some(inner_event_handler) = &self.inner_event_handler {
			let event_interface = PyEventCallbackInterface::new(inner_event_handler.clone());
			let event_handler: PyObject =
				Py::new(py, event_interface).expect("Unable to create class").into_py(py);
			neo4j_query_dict.set_item("event_handler", event_handler).unwrap();
		}
		// convert token feeder to python object
		if let Some(inner_tokens_feader) = &self.inner_tokens_feader {
			let channel_interface = PyMessageInterface::new(inner_tokens_feader.clone());
			let tokens_feader: PyObject =
				Py::new(py, channel_interface).expect("Unable to create class").into_py(py);
			neo4j_query_dict.set_item("tokens_feader", tokens_feader).unwrap();
		}
		neo4j_query_dict.into()
	}
}
