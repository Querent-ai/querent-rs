use crate::config::Config;
use pyo3::prelude::*;
use std::{collections::HashMap, sync::Mutex};
use tokio::{sync::mpsc, task::spawn_blocking};

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
	running_workflows: Mutex<HashMap<String, tokio::task::JoinHandle<Result<(), String>>>>,
}

impl WorkflowManager {
	/// Creates a new `WorkflowManager` instance.
	pub fn new() -> Self {
		WorkflowManager {
			workflows: Mutex::new(HashMap::new()),
			running_workflows: Mutex::new(HashMap::new()),
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
		if workflows.contains_key(&workflow.id) {
			return Err("Workflow with the same ID already exists.".to_string())
		} else {
			workflows.insert(workflow.id.clone(), workflow.clone());
		}
		Ok(())
	}

	/// Removes a workflow from the manager.
	///
	/// # Arguments
	///
	/// * `id` - The ID of the workflow to be removed.
	///
	/// # Returns
	///
	/// Returns a `Result` indicating success or an error message.
	pub fn remove_workflow(&self, id: &str) -> Result<(), String> {
		let mut workflows = self.workflows.lock().map_err(|_| "Mutex lock failed.".to_string())?;
		if self.running_workflows.lock().unwrap().contains_key(id) {
			return Err("Workflow is running.".to_string())
		}
		if workflows.remove(id).is_some() {
			Ok(())
		} else {
			Err("Workflow not found.".to_string())
		}
	}

	/// Retrieves a workflow by ID.
	///
	/// # Arguments
	///
	/// * `id` - The ID of the workflow to be retrieved.
	///
	/// # Returns
	///
	/// Returns an `Option` containing the retrieved workflow or `None`.
	pub fn get_workflow(&self, id: &str) -> Option<Workflow> {
		let workflows = self.workflows.lock().ok()?;
		workflows.get(id).cloned()
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

	/// Checks if a workflow with the given ID is currently running.
	///
	/// # Arguments
	///
	/// * `id` - The ID of the workflow to be checked.
	///
	/// # Returns
	///
	/// Returns `true` if the workflow is running; otherwise, returns `false`.
	pub fn is_running(&self, id: &str) -> bool {
		let running_workflows = self.running_workflows.lock().unwrap();
		running_workflows.contains_key(id)
	}

	/// Starts a workflow by executing its Python code asynchronously.
	///
	/// # Arguments
	///
	/// * `id` - The ID of the workflow to be started.
	/// * `tx` - A channel sender for status messages.
	///
	/// # Returns
	///
	/// Returns a `Result` indicating success or an error message.
	pub async fn start_workflow(&self, id: String, tx: mpsc::Sender<String>) -> Result<(), String> {
		let workflow = self.get_workflow(&id).ok_or("Workflow not found.")?;
		if self.is_running(&id) {
			return Err("Workflow is already running.".to_string())
		}
		pyo3::prepare_freethreaded_python();

		let config = workflow.config.clone();
		let python_import_path = workflow.python_import_path.clone();
		let python_start_function = workflow.python_start_function.clone();
		let id_clone = id.clone();
		let handle = spawn_blocking(move || {
			Python::with_gil(|py| {
				println!("Starting workflow {}...", id_clone.clone());
				let querent_start_workflow = py
					.import(python_import_path.as_str())
					.map_err(|_| "Failed to import workflow.");
				println!("querent_start_workflow: {:?}", querent_start_workflow);
				let result = querent_start_workflow
					.unwrap()
					.call_method1(python_start_function.as_str(), (config,))
					.map_err(|_| "Failed to start workflow.");
				println!("result: {:?}", result);
				match result {
					Ok(_) => {
						let _ = tx.send(format!("Workflow {} has started.", id_clone.clone()));
						Ok(())
					},
					Err(_) => {
						let _ = tx.send(format!("Failed to start Workflow {}.", id_clone.clone()));
						Err("Failed to start workflow.".to_string())
					},
				}
			})
		});

		let mut running_workflows = self.running_workflows.lock().unwrap();
		running_workflows.insert(id, handle);

		Ok(())
	}

	/// Stops a running workflow by executing its Python code asynchronously.
	///
	/// # Arguments
	///
	/// * `id` - The ID of the workflow to be stopped.
	/// * `tx` - A channel sender for status messages.
	///
	/// # Returns
	///
	/// Returns a `Result` indicating success or an error message.
	pub async fn stop_workflow(&self, id: String, tx: mpsc::Sender<String>) -> Result<(), String> {
		let workflow = self.get_workflow(&id).ok_or("Workflow not found.")?;
		if !self.is_running(&id) {
			return Err("Workflow is not running.".to_string())
		}

		let handle = self.running_workflows.lock().unwrap().remove(&id).unwrap();
		let _result = handle.await.map_err(|_| "Failed to stop workflow.".to_string())?;

		let python_import_path = workflow.python_import_path.clone();
		let id_clone = id.clone();
		let python_stop_function = workflow.python_stop_function.clone();
		let result = spawn_blocking(move || {
			Python::with_gil(|py| {
				let querent_stop_workflow = py
					.import(python_import_path.as_str())
					.map_err(|_| "Failed to import workflow.")?;
				let result = querent_stop_workflow
					.call_method0(python_stop_function.as_str())
					.map_err(|_| "Failed to stop workflow.");

				match result {
					Ok(_) => {
						let _ = tx.send(format!("Workflow {} has stopped.", id_clone));
						Ok(())
					},
					Err(_) => {
						let _ = tx.send(format!("Failed to stop Workflow {}.", id_clone));
						Err("Failed to stop workflow.".to_string())
					},
				}
			})
		})
		.await
		.map_err(|_| "Failed to stop workflow.".to_string())?;

		result
	}
}
