use crate::{
	callbacks::{EventCallbackInterface, PyEventCallbackInterface},
	comm::ChannelHandler,
	config::Config,
	cross::{CLRepr, CLReprPython},
	querent::{py_runtime, PyRuntime, QuerentError},
	tokio_runtime,
};
use futures::TryFutureExt;
use log;
use pyo3::{
	prelude::*,
	types::{PyDict, PyFunction},
};
use std::{collections::HashMap, sync::Mutex};
use tokio::runtime::Runtime;

/// Represents a workflow.
#[derive(Debug, Clone)]
#[pyclass]
pub struct Workflow {
	/// Name of the workflow.
	pub name: String,
	/// Unique identifier for the workflow.
	pub id: String,
	/// Python module to import for the workflow.
	pub import: String,
	/// Attribute of the Python module containing the start function.
	pub attr: String,
	/// Optional Python code to execute instead of importing a module.
	pub code: Option<String>,
	/// Arguments to pass to the workflow's start function.
	pub arguments: Vec<CLRepr>,
	/// Optional configuration for the workflow.
	pub config: Option<Config>,
}

/// Manages workflows and their execution.
pub struct WorkflowManager {
	/// Mutex-protected map of workflows, keyed by their unique identifier.
	pub workflows: Mutex<HashMap<String, Workflow>>,
	/// Reference to the Python runtime.
	pub runtime: &'static PyRuntime,
}

impl WorkflowManager {
	/// Creates a new `WorkflowManager` instance.
	pub fn new() -> Result<Self, String> {
		let runtime = py_runtime().map_err(|e| e.to_string())?;
		Ok(Self { workflows: Mutex::new(HashMap::new()), runtime })
	}

	/// Adds a workflow to the manager.
	pub fn add_workflow(&self, workflow: Workflow) -> Result<(), String> {
		let mut workflows =
			self.workflows.lock().map_err(|e| format!("Mutex lock failed: {}", e))?;
		if workflows.contains_key(&workflow.id) {
			return Err("Workflow with the same ID already exists.".to_string());
		} else {
			workflows.insert(workflow.id.clone(), workflow.clone());
		}
		Ok(())
	}

	/// Retrieves a list of all workflows managed by this manager.
	pub fn get_workflows(&self) -> Vec<Workflow> {
		let workflows = self.workflows.lock().unwrap();
		workflows.values().cloned().collect()
	}

	/// Starts workflows by executing their Python code asynchronously.
	pub async fn start_workflows(&self) -> Result<(), QuerentError> {
		let workflows = self.get_workflows();
		let handles: Vec<_> = workflows
			.iter()
			.map(|_workflow| {
				let args = _workflow.arguments.clone();
				let res = match &_workflow.code {
					None => Python::with_gil(|py| {
						let async_mod = py.import(_workflow.import.as_str()).map_err(|e| {
							log::error!("Failed to import module {}: {}", _workflow.import, e);
							QuerentError::internal(e.to_string())
						})?;

						let coroutine =
							async_mod.getattr(_workflow.attr.as_str()).map_err(|_| {
								log::error!("Failed to find start function.");
								QuerentError::internal("Failed to find start function.".to_string())
							})?;

						let querent_py_fun: Py<PyFunction> = coroutine.extract().map_err(|e| {
							log::error!("Failed to extract function: {}", e);
							QuerentError::internal(e.to_string())
						})?;

						let mut config_pyobject: Option<PyObject> = None;
						if let Some(config) = &_workflow.config {
							config_pyobject = Some(config.to_object(py));
						}
						let call_future =
							self.runtime.call_async(querent_py_fun, args, config_pyobject);
						Ok(call_future)
					}),
					Some(code) => {
						let module_file: String = _workflow.id.clone() + ".py";
						Python::with_gil(|py| {
							let dynamic_module = PyModule::from_code(
								py,
								code.as_str(),
								module_file.as_str(),
								_workflow.name.as_str(),
							)
							.map_err(|e| {
								log::error!("Failed to import module {}: {}", _workflow.import, e);
								QuerentError::internal(e.to_string())
							})?;

							let attr_fun =
								dynamic_module.getattr(_workflow.attr.as_str()).map_err(|_| {
									log::error!("Failed to find start function.");
									QuerentError::internal(
										"Failed to find start function.".to_string(),
									)
								})?;

							let querent_py_fun: Py<PyFunction> =
								attr_fun.extract().map_err(|e| {
									log::error!("Failed to extract function: {}", e);
									QuerentError::internal(e.to_string())
								})?;

							let mut config_pyobject: Option<PyObject> = None;
							if let Some(config) = &_workflow.config {
								config_pyobject = Some(config.to_object(py));
							}
							let call_future =
								self.runtime.call_async(querent_py_fun, args, config_pyobject);
							Ok(call_future)
						})
					},
				};
				res
			})
			.collect();
		for handle in handles {
			match handle {
				Ok(future) => match future.await {
					Ok(_) => log::info!("Workflow started."),
					Err(e) => {
						log::error!("Failed to start workflow: {}", e);
						return Err(QuerentError::internal(e.to_string()));
					},
				},
				Err(e) => {
					log::error!("Failed to start workflow: {}", e);
					return Err(e);
				},
			}
		}
		Ok(())
	}
}

impl Drop for WorkflowManager {
	/// Drops the `WorkflowManager` instance, cleaning up resources.
	fn drop(&mut self) {
		log::info!("Dropping WorkflowManager");
		let _ = self.runtime;
	}
}
