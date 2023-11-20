use crate::{
	callbacks::{EventCallbackInterface, PyEventCallbackInterface},
	config::Config,
	cross::{CLRepr, CLReprPython},
	querent::{py_runtime, PyRuntime, QuerentError},
	tokio_runtime,
};
use futures::TryFutureExt;
use log;
use pyo3::{prelude::*, types::PyFunction};
use std::{collections::HashMap, sync::Mutex};
use tokio::runtime::Runtime;

/// Represents a workflow.
#[derive(Debug, Clone)]
pub struct Workflow {
	pub name: String,
	pub id: String,
	pub import: String,
	pub attr: String,
	pub code: Option<String>,
	pub arguments: Vec<CLRepr>,
	pub config: Option<Config>,
	pub event_callback: Option<PyEventCallbackInterface>,
}

/// Manages workflows and their execution.
pub struct WorkflowManager {
	pub workflows: Mutex<HashMap<String, Workflow>>,
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
			return Err("Workflow with the same ID already exists.".to_string())
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

	/// Starts a workflow by executing its Python code asynchronously.
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

						let call_future =
							self.runtime.call_async(querent_py_fun, args, _workflow.config.clone());
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

							let call_future = self.runtime.call_async(
								querent_py_fun,
								args,
								_workflow.config.clone(),
							);
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
						return Err(QuerentError::internal(e.to_string()))
					},
				},
				Err(e) => {
					log::error!("Failed to start workflow: {}", e);
					return Err(e)
				},
			}
		}
		Ok(())
	}
}

impl Drop for WorkflowManager {
	fn drop(&mut self) {
		log::info!("Dropping WorkflowManager");
		let _ = self.runtime;

		// cleanup the Python runtime
		Python::with_gil(|py| {
			if let Err(e) = py
				.import("sys")
				.and_then(|sys| sys.getattr("exit"))
				.and_then(|exit| exit.call0())
			{
				log::error!("Failed to exit Python runtime: {}", e);
			}
		});
	}
}
