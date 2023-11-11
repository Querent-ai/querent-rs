use crate::{
	config::Config,
	cross::CLRepr,
	querent::{py_runtime, PyRuntime, QuerentError},
	tokio_runtime,
};
use log;
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
	pub import: String,
	pub attr: String,
	pub arguments: Vec<CLRepr>,
	pub code: Option<String>,
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
	pub async fn start_workflows(&self) -> Result<(), QuerentError> {
		let workflows = self.get_workflows();

		for _workflow in workflows {
			let res = Python::with_gil(|py| {
				let asyncio = py.import("asyncio")?;

				// Convert asyncio.sleep into a Rust Future
				let sleep_future = pyo3_asyncio::tokio::into_future(
					asyncio.call_method1("sleep", (1.into_py(py),))?,
				);

				sleep_future
			})
			.map_err(|e| QuerentError::internal(e.to_string()))?;

			res.await.map_err(|e| QuerentError::internal(e.to_string()))?;
		}
		Ok(())
	}
}
