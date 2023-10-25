use std::collections::HashMap;

use pyo3::{prelude::*, types::PyDict, PyObject, ToPyObject};

#[derive(Debug)]
#[pyclass]
pub struct Config {
	pub version: f32,
	pub querent_id: String,
	pub querent_name: String,
	pub workflow: WorkflowConfig,
	pub collectors: Vec<CollectorConfig>,
	pub engines: Vec<EngineConfig>,
	pub resource: Option<ResourceConfig>,
}

#[derive(Debug)]
#[pyclass]
pub struct WorkflowConfig {
	pub name: String,
	pub id: String,
	pub config: HashMap<String, String>,
}

impl<'a> FromPyObject<'a> for WorkflowConfig {
	fn extract(ob: &'a PyAny) -> PyResult<Self> {
		let name = ob.getattr("name")?.extract()?;
		let id = ob.getattr("id")?.extract()?;
		let config = ob.getattr("config")?.extract()?;
		Ok(WorkflowConfig { name, id, config })
	}
}

impl ToPyObject for WorkflowConfig {
	fn to_object(&self, py: Python) -> PyObject {
		let workflow_dict = PyDict::new(py);
		workflow_dict.set_item("name", &self.name).unwrap();
		workflow_dict.set_item("id", &self.id).unwrap();
		workflow_dict.set_item("config", &self.config).unwrap();

		workflow_dict.to_object(py)
	}
}

#[derive(Debug)]
pub struct CollectorConfig {
	name: String,
	backend: String,
	config: HashMap<String, String>,
}

impl<'a> FromPyObject<'a> for CollectorConfig {
	fn extract(ob: &'a PyAny) -> PyResult<Self> {
		let name = ob.getattr("name")?.extract()?;
		let backend = ob.getattr("backend")?.extract()?;
		let config = ob.getattr("config")?.extract()?;
		Ok(CollectorConfig { name, backend, config })
	}
}

impl ToPyObject for CollectorConfig {
	fn to_object(&self, py: Python) -> PyObject {
		let collector_dict = PyDict::new(py);
		collector_dict.set_item("name", &self.name).unwrap();
		collector_dict.set_item("backend", &self.backend).unwrap();
		collector_dict.set_item("config", &self.config).unwrap();

		collector_dict.to_object(py)
	}
}

#[derive(Debug)]
pub struct EngineConfig {
	name: String,
	num_workers: Option<u32>,
	max_retries: Option<u32>,
	retry_interval: Option<u32>,
	message_throttle_limit: Option<u32>,
	message_throttle_delay: Option<u32>,
}

impl<'a> FromPyObject<'a> for EngineConfig {
	fn extract(ob: &'a PyAny) -> PyResult<Self> {
		let name = ob.getattr("name")?.extract()?;
		let num_workers = ob.getattr("num_workers")?.extract()?;
		let max_retries = ob.getattr("max_retries")?.extract()?;
		let retry_interval = ob.getattr("retry_interval")?.extract()?;
		let message_throttle_limit = ob.getattr("message_throttle_limit")?.extract()?;
		let message_throttle_delay = ob.getattr("message_throttle_delay")?.extract()?;
		Ok(EngineConfig {
			name,
			num_workers,
			max_retries,
			retry_interval,
			message_throttle_limit,
			message_throttle_delay,
		})
	}
}

impl ToPyObject for EngineConfig {
	fn to_object(&self, py: Python) -> PyObject {
		let engine_dict = PyDict::new(py);
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

		engine_dict.to_object(py)
	}
}

#[derive(Debug)]
pub struct ResourceConfig {
	max_workers_allowed: Option<u32>,
	max_workers_per_collector: Option<u32>,
	max_workers_per_engine: Option<u32>,
	max_workers_per_querent: Option<u32>,
}

impl<'a> FromPyObject<'a> for ResourceConfig {
	fn extract(ob: &'a PyAny) -> PyResult<Self> {
		let max_workers_allowed = ob.getattr("max_workers_allowed")?.extract()?;
		let max_workers_per_collector = ob.getattr("max_workers_per_collector")?.extract()?;
		let max_workers_per_engine = ob.getattr("max_workers_per_engine")?.extract()?;
		let max_workers_per_querent = ob.getattr("max_workers_per_querent")?.extract()?;
		Ok(ResourceConfig {
			max_workers_allowed,
			max_workers_per_collector,
			max_workers_per_engine,
			max_workers_per_querent,
		})
	}
}

impl ToPyObject for ResourceConfig {
	fn to_object(&self, py: Python) -> PyObject {
		let resource_dict = PyDict::new(py);
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

impl ToPyObject for Config {
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
