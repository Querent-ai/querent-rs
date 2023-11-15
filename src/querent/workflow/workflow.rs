use crate::{
	config::Config,
	cross::{CLRepr, CLReprPython},
	querent::{py_runtime, PyRuntime, QuerentError},
	tokio_runtime,
};
use core::panic;
use futures::TryFutureExt;
use log;
use pyo3::{
	prelude::*,
	types::{PyDict, PyFunction, PyTuple},
};
use std::{collections::HashMap, sync::Mutex};
use tokio::{runtime::Runtime, task::spawn_blocking};

/// Represents a workflow.
#[derive(Debug, Clone)]
pub struct Workflow {
	pub name: String,
	pub id: String,
	pub import: String,
	pub attr: String,
	pub arguments: Vec<CLRepr>,
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
	///
	/// # Arguments
	///
	/// * `workflow` - The workflow to be added.
	///
	/// # Returns
	///
	/// Returns a `Result` indicating success or an error message.
	pub fn add_workflow(&self, workflow: Workflow) -> Result<(), String> {
		let mut workflows = self.workflows.lock().map_err(|_| "Mutex lock failed.".to_string())?;
		if workflows.len() >= 1 {
			return Err("Only one workflow is supported.".to_string())
		}
		if workflows.contains_key(&workflow.id) {
			return Err("Workflow with the same ID already exists.".to_string())
		} else {
			workflows.insert(workflow.id.clone(), workflow.clone());
		}
		Ok(())
	}

	/// Retrieves a list of all workflows managed by this manager.
	///
	/// # Returns
	///
	/// Returns a `Vec` containing all the managed workflows.
	pub fn get_workflows(&self) -> Vec<Workflow> {
		let workflows = self.workflows.lock().ok().unwrap();
		workflows.values().cloned().collect()
	}

	/// Starts a workflow by executing its Python code asynchronously.
	///
	/// # Returns
	///
	/// Returns a `Result` indicating success or an error message.
	pub async fn start_workflows(&self) -> Result<(), QuerentError> {
		let workflows = self.get_workflows();
		let _tokio = tokio_runtime().map_err(|e| QuerentError::internal(e.to_string()))?;
		let handles: Vec<_> = workflows
			.iter()
			.map(|_workflow| {
				let args = _workflow.arguments.clone();
				let res = Python::with_gil(|py| {
					let async_mod = py
						.import(_workflow.import.as_str())
						.map_err(|e| {
							log::error!("Failed to import module {}: {}", _workflow.import, e);
							QuerentError::internal(e.to_string())
						})
						.expect("Failed to import module.");

					// Get the Python function
					let coroutine = async_mod
						.getattr(_workflow.attr.as_str())
						.map_err(|_| "Failed to find start function.")
						.expect("Failed to find start function.");

					let querent_py_fun: Py<PyFunction> =
						coroutine.extract().expect("Failed to extract function.");
					let call_future = self.runtime.call_async(querent_py_fun, args);
					call_future
				});
				// use tokio to spawn a future
				_tokio.spawn(res)
			})
			.collect();

		// Wait for all the tasks to finish
		for handle in handles {
			let _ = handle.await.map_err(|e| QuerentError::internal(e.to_string()))?;
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
			let sys = py.import("sys").expect("Failed to import sys module");
			let exit = sys.getattr("exit").expect("sys module does not have attribute exit");
			let _ = exit.call0();
		});
	}
}
