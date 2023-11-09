use crate::{
	config::Config,
	querent::{py_runtime, PyRuntime},
};
use pyo3::{
	prelude::*,
	types::{PyDict, PyFunction},
};
use std::{collections::HashMap, sync::Mutex};
use tokio::{runtime::Runtime, task::spawn_blocking};

/// Represents a workflow.
#[derive(Debug, Clone)]
pub struct Workflow {
	pub name: String,
	pub id: String,
	pub python_import_path: String,
	pub python_start_function: String,
	pub arguments: Config,
}

/// Manages workflows and their execution.
pub struct WorkflowManager {
	pub workflows: Mutex<HashMap<String, Workflow>>,
	pub runtime: &'static PyRuntime,
}

impl WorkflowManager {
	/// Creates a new `WorkflowManager` instance.
	pub fn new() -> Self {
		let runtime = py_runtime();
		match runtime {
			Ok(runtime) => Self { workflows: Mutex::new(HashMap::new()), runtime },
			Err(_) => panic!("Failed to create Python runtime."),
		}
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
	pub async fn start_workflows(&self) -> Result<(), String> {
		let workflows = self.get_workflows();
		let handles: Vec<_> = workflows
			.iter()
			.map(|workflow| {
				let args = workflow.arguments.clone();
				let python_import_path = workflow.python_import_path.clone();
				let python_start_function = workflow.python_start_function.clone();
				let runtime_local = self.runtime;

				spawn_blocking(move || {
					Python::with_gil(|py| {
						// Import the Python module
						let querent_start_workflow = py
							.import(python_import_path.as_str())
							.map_err(|_| "Failed to import workflow.")
							.unwrap(); // You may want to handle this error more gracefully

						// Get the Python function
						let coroutine = querent_start_workflow
							.getattr(python_start_function.as_str())
							.map_err(|_| "Failed to find start function.")
							.unwrap(); // You may want to handle this error more gracefully

						// Call the Python function with arguments
						let result = coroutine.call1((args,)).unwrap(); // You may want to handle this error more gracefully

						// Convert the result into a Py<PyFunction>
						let querent_py_fun: Py<PyFunction> = result.extract().unwrap(); // You may want to handle this error more gracefully

						// Call your runtime's call_async function with the PyFunction and arguments
						runtime_local.call_async(querent_py_fun, Vec::new())
					})
				})
			})
			.collect();

		for handle in handles {
			let result = handle.await.map_err(|_| "Failed to start workflow.".to_string())?;
			match result.await {
				Ok(_) => {
					println!("Workflow started.");
					println!("Waiting for workflow to complete...");
				},
				Err(_) => return Err("Failed to start workflow.".to_string()),
			}
		}
		Ok(())
	}
}
