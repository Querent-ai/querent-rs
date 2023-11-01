use crate::config::Config;
use pyo3::prelude::*;
use std::{collections::HashMap, sync::Mutex};
use tokio::task::spawn_blocking;

/// Represents a workflow.
#[derive(Debug, Clone)]
pub struct Workflow {
	pub name: String,
	pub id: String,
	pub python_import_path: String,
	pub python_start_function: String,
	pub python_stop_function: String,
	pub config: Config,
}

/// Manages workflows and their execution.
pub struct WorkflowManager {
	workflows: Mutex<HashMap<String, Workflow>>,
}

impl WorkflowManager {
	/// Creates a new `WorkflowManager` instance.
	pub fn new() -> Self {
		WorkflowManager { workflows: Mutex::new(HashMap::new()) }
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
				let config = workflow.config.clone();
				let python_import_path = workflow.python_import_path.clone();
				let python_start_function = workflow.python_start_function.clone();
				spawn_blocking(move || {
					Python::with_gil(|py| {
						let querent_start_workflow = py
							.import(python_import_path.as_str())
							.map_err(|_| "Failed to import workflow.");
						if querent_start_workflow.is_err() {
							panic!("Failed to import workflow.")
						}
						let coroutine = querent_start_workflow
							.unwrap()
							.call_method1(python_start_function.as_str(), (config,))
							.map_err(|_| "Failed to start workflow.")?;

						pyo3_asyncio::tokio::into_future(coroutine)
							.map_err(|_| "Failed to start workflow.")
					})
				})
			})
			.collect();

		for handle in handles {
			let result = handle.await.map_err(|_| "Failed to start workflow.".to_string())?;
			if result.is_err() {
				return Err("Failed to start workflow.".to_string())
			}

			let result = result.unwrap();
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

	/// Stops a running workflow by executing its Python code asynchronously.
	///
	/// # Returns
	///
	/// Returns a `Result` indicating success or an error message.
	pub async fn stop_workflows(&self) -> Result<(), String> {
		let workflows = self.get_workflows();
		let handles: Vec<_> = workflows
			.iter()
			.map(|workflow| {
				let python_import_path = workflow.python_import_path.clone();
				let python_stop_function = workflow.python_stop_function.clone();
				spawn_blocking(move || {
					Python::with_gil(|py| {
						let querent_stop_workflow = py
							.import(python_import_path.as_str())
							.map_err(|_| "Failed to import workflow.");
						let coroutine = querent_stop_workflow
							.unwrap()
							.call_method0(python_stop_function.as_str())
							.map_err(|_| "Failed to stop workflow.");

						pyo3_asyncio::tokio::into_future(coroutine.unwrap())
							.map_err(|_| "Failed to stop workflow.")
					})
				})
			})
			.collect();

		for handle in handles {
			let result = handle.await.map_err(|_| "Failed to stop workflow.".to_string())?;
			if result.is_err() {
				return Err("Failed to stop workflow.".to_string())
			}
		}
		Ok(())
	}
}
