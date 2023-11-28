use std::collections::HashMap;

use pyo3::{prelude::*, types::PyDict, PyObject, ToPyObject};

use crate::{
	callbacks::interface::EventHandler,
	comm::{ChannelHandler, PyMessageInterface},
};

/// Configuration struct representing the overall setup for a system.
#[derive(Debug, Clone)]
#[pyclass]
pub struct Config {
	/// Version of the configuration format.
	pub version: f32,
	/// Unique identifier for the querent (user/client).
	pub querent_id: String,
	/// Name of the querent.
	pub querent_name: String,
	/// Configuration for the workflow.
	pub workflow: WorkflowConfig,
	/// List of collector configurations.
	pub collectors: Vec<CollectorConfig>,
	/// List of engine configurations.
	pub engines: Vec<EngineConfig>,
	/// Optional resource configuration.
	pub resource: Option<ResourceConfig>,
}

impl ToPyObject for Config {
	/// Converts a Config to a Python object.
	fn to_object(&self, py: Python) -> PyObject {
		let config_dict = PyDict::new(py);
		config_dict.set_item("version", self.version).unwrap();
		config_dict.set_item("querent_id", &self.querent_id).unwrap();
		config_dict.set_item("querent_name", &self.querent_name).unwrap();
		config_dict.set_item("workflow", &self.workflow).unwrap();
		config_dict.set_item("collectors", &self.collectors).unwrap();
		config_dict.set_item("engines", &self.engines).unwrap();
		config_dict.set_item("resource", &self.resource).unwrap();

		config_dict.to_object(py)
	}
}

impl Default for Config {
	/// Creates a default configuration.
	fn default() -> Self {
		Config {
			version: 0.1,
			querent_id: "querent".to_string(),
			querent_name: "Querent".to_string(),
			workflow: WorkflowConfig {
				name: "workflow".to_string(),
				id: "workflow".to_string(),
				config: HashMap::new(),
				inner_channel: ChannelHandler::new(),
				channel: None,
				inner_event_handler: EventHandler::new(),
				event_handler: None,
			},
			collectors: vec![],
			engines: vec![],
			resource: None,
		}
	}
}

/// Configuration for a workflow.
#[derive(Debug, Clone)]
#[pyclass]
pub struct WorkflowConfig {
	/// Name of the workflow.
	pub name: String,
	/// Unique identifier for the workflow.
	pub id: String,
	/// Additional configuration options for the workflow.
	pub config: HashMap<String, String>,
	/// Internal channel handler in rust will be wrapped and marshalled into python.
	pub inner_channel: ChannelHandler,
	/// PyObject for the channel handler.
	/// This is a workaround for the fact that PyMessageInterface is not a PyObject.
	#[pyo3(get, set)]
	pub channel: Option<PyObject>,
	/// Inner EventHandler for workflow to get events from python
	pub inner_event_handler: EventHandler,
	/// PyObject for the event handler.
	#[pyo3(get, set)]
	pub event_handler: Option<PyObject>,
}

impl ToPyObject for WorkflowConfig {
	/// Converts a WorkflowConfig to a Python object.
	fn to_object(&self, py: Python) -> PyObject {
		let workflow_dict = PyDict::new(py);
		workflow_dict.set_item("name", &self.name).unwrap();
		workflow_dict.set_item("id", &self.id).unwrap();
		workflow_dict.set_item("config", &self.config).unwrap();
		// convert channel handler to python object
		let channel_interface = PyMessageInterface::new(self.inner_channel.clone());
		let channel: PyObject =
			Py::new(py, channel_interface).expect("Unable to create class").into_py(py);
		workflow_dict.set_item("channel", channel).unwrap();

		workflow_dict.to_object(py)
	}
}

/// Configuration for a collector.
#[derive(Debug, Clone)]
#[pyclass]
pub struct CollectorConfig {
	/// Unique identifier for the collector.
	pub id: String,
	/// Name of the collector.
	pub name: String,
	/// Backend used by the collector.
	pub backend: String,
	/// Additional configuration options for the collector.
	pub config: HashMap<String, String>,
	/// Internal channel handler in rust will be wrapped and marshalled into python.
	pub inner_channel: ChannelHandler,
	/// PyObject for the channel handler.
	#[pyo3(get, set)]
	pub channel: Option<PyObject>,
}

impl ToPyObject for CollectorConfig {
	/// Converts a CollectorConfig to a Python object.
	fn to_object(&self, py: Python) -> PyObject {
		let collector_dict = PyDict::new(py);
		collector_dict.set_item("id", &self.id).unwrap();
		collector_dict.set_item("name", &self.name).unwrap();
		collector_dict.set_item("backend", &self.backend).unwrap();
		collector_dict.set_item("config", &self.config).unwrap();
		// convert channel handler to python object
		let channel_interface = PyMessageInterface::new(self.inner_channel.clone());
		let channel: PyObject =
			Py::new(py, channel_interface).expect("Unable to create class").into_py(py);
		collector_dict.set_item("channel", channel).unwrap();
		collector_dict.to_object(py)
	}
}

/// Configuration for an engine.
#[derive(Debug, Clone)]
#[pyclass]
pub struct EngineConfig {
	/// Unique identifier for the engine.
	pub id: String,
	/// Name of the engine.
	pub name: String,
	/// Number of workers used by the engine (optional).
	pub num_workers: Option<u32>,
	/// Maximum number of retries for the engine (optional).
	pub max_retries: Option<u32>,
	/// Interval between retries for the engine (optional).
	pub retry_interval: Option<u32>,
	/// Message throttle limit for the engine (optional).
	pub message_throttle_limit: Option<u32>,
	/// Message throttle delay for the engine (optional).
	pub message_throttle_delay: Option<u32>,
	/// Internal channel handler in rust will be wrapped and marshalled into python.
	pub inner_channel: ChannelHandler,
	/// PyObject for the channel handler.
	#[pyo3(get, set)]
	pub channel: Option<PyObject>,
}

impl ToPyObject for EngineConfig {
	/// Converts an EngineConfig to a Python object.
	fn to_object(&self, py: Python) -> PyObject {
		let engine_dict = PyDict::new(py);
		engine_dict.set_item("id", &self.id).unwrap();
		engine_dict.set_item("name", &self.name).unwrap();
		engine_dict.set_item("num_workers", &self.num_workers).unwrap();
		engine_dict.set_item("max_retries", &self.max_retries).unwrap();
		engine_dict.set_item("retry_interval", &self.retry_interval).unwrap();
		engine_dict
			.set_item("message_throttle_limit", &self.message_throttle_limit)
			.unwrap();
		engine_dict
			.set_item("message_throttle_delay", &self.message_throttle_delay)
			.unwrap();
		// convert channel handler to python object
		let channel_interface = PyMessageInterface::new(self.inner_channel.clone());
		let channel: PyObject =
			Py::new(py, channel_interface).expect("Unable to create class").into_py(py);
		engine_dict.set_item("channel", channel).unwrap();

		engine_dict.to_object(py)
	}
}

/// Configuration for resource constraints.
#[derive(Debug, Clone)]
pub struct ResourceConfig {
	/// Unique identifier for the resource.
	pub id: String,
	/// Maximum number of workers allowed (optional).
	pub max_workers_allowed: Option<u32>,
	/// Maximum number of workers per collector (optional).
	pub max_workers_per_collector: Option<u32>,
	/// Maximum number of workers per engine (optional).
	pub max_workers_per_engine: Option<u32>,
	/// Maximum number of workers per querent (optional).
	pub max_workers_per_querent: Option<u32>,
}

// Implementation of conversion traits for ResourceConfig.
impl<'a> FromPyObject<'a> for ResourceConfig {
	/// Extracts a ResourceConfig from a Python object.
	fn extract(ob: &'a PyAny) -> PyResult<Self> {
		let id = ob.getattr("id")?.extract()?;
		let max_workers_allowed = ob.getattr("max_workers_allowed")?.extract()?;
		let max_workers_per_collector = ob.getattr("max_workers_per_collector")?.extract()?;
		let max_workers_per_engine = ob.getattr("max_workers_per_engine")?.extract()?;
		let max_workers_per_querent = ob.getattr("max_workers_per_querent")?.extract()?;
		Ok(ResourceConfig {
			id,
			max_workers_allowed,
			max_workers_per_collector,
			max_workers_per_engine,
			max_workers_per_querent,
		})
	}
}

impl ToPyObject for ResourceConfig {
	/// Converts a ResourceConfig to a Python object.
	fn to_object(&self, py: Python) -> PyObject {
		let resource_dict = PyDict::new(py);
		resource_dict.set_item("id", &self.id).unwrap();
		resource_dict
			.set_item("max_workers_allowed", &self.max_workers_allowed)
			.unwrap();
		resource_dict
			.set_item("max_workers_per_collector", &self.max_workers_per_collector)
			.unwrap();
		resource_dict
			.set_item("max_workers_per_engine", &self.max_workers_per_engine)
			.unwrap();
		resource_dict
			.set_item("max_workers_per_querent", &self.max_workers_per_querent)
			.unwrap();

		resource_dict.to_object(py)
	}
}

#[pymethods]
impl Config {
	/// Constructor for creating a new Config instance.
	#[new]
	fn new(
		version: f32,
		querent_id: String,
		querent_name: String,
		workflow: WorkflowConfig,
		collectors: Vec<CollectorConfig>,
		engines: Vec<EngineConfig>,
		resource: Option<ResourceConfig>,
	) -> Self {
		Config { version, querent_id, querent_name, workflow, collectors, engines, resource }
	}
}
