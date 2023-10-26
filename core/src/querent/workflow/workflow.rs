use crate::config::Config;
use pyo3::prelude::*;
use std::{collections::HashMap, sync::Mutex};

#[derive(Debug, Clone)]
pub struct Workflow {
	pub name: String,
	pub id: String,
	pub python_import_path: String,
	pub python_start_function: String,
	pub config: Config,
}

pub struct WorkflowManager {
	workflows: Mutex<HashMap<String, Workflow>>,
	running_workflows: Mutex<HashMap<String, Workflow>>,
}

impl WorkflowManager {
	pub fn new() -> Self {
		WorkflowManager {
			workflows: Mutex::new(HashMap::new()),
			running_workflows: Mutex::new(HashMap::new()),
		}
	}

	pub fn add_workflow(&self, workflow: Workflow) -> Result<(), String> {
		let mut workflows = self.workflows.lock().map_err(|_| "Mutex lock failed.".to_string())?;
		if workflows.contains_key(&workflow.id) {
			return Err("Workflow with the same ID already exists.".to_string())
		} else {
			workflows.insert(workflow.id.clone(), workflow.clone());
		}
		Ok(())
	}

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

	pub fn get_workflow(&self, id: &str) -> Option<Workflow> {
		let workflows = self.workflows.lock().ok()?;
		workflows.get(id).cloned()
	}

	pub fn get_workflows(&self) -> Vec<Workflow> {
		let workflows = self.workflows.lock().ok().unwrap();
		workflows.values().cloned().collect()
	}

	pub fn is_running(&self, id: &str) -> bool {
		let running_workflows = self.running_workflows.lock().unwrap();
		running_workflows.contains_key(id)
	}
}

impl WorkflowManager {
	pub async fn start_workflow(&self, id: &str) -> Result<(), String> {
		let workflow = self.get_workflow(id).ok_or("Workflow not found.")?;
		if self.is_running(id) {
			return Err("Workflow is already running.".to_string())
		}

		let config = workflow.config.clone();

		let workflow_future = Python::with_gil(|py| {
			let querent_start_workflow = py
				.import(workflow.python_import_path.as_str())
				.map_err(|_| "Failed to import workflow.")?;
			let inner_fut = pyo3_asyncio::async_std::into_future(
				querent_start_workflow
					.call_method1(workflow.python_start_function.as_str(), (config,))
					.map_err(|_| "Failed to start workflow.")?,
			);
			if let Ok(fut) = inner_fut {
				Ok(fut)
			} else {
				Err("Failed to start workflow.".to_string())
			}
		})
		.map_err(|_| "Failed to start workflow.".to_string());

		let workflow_future = workflow_future?;

		match workflow_future.await {
			Ok(_) => {
				let mut running_workflows = self.running_workflows.lock().unwrap();
				running_workflows.insert(id.to_string(), workflow);
			},
			Err(_) => return Err("Failed to start workflow.".to_string()),
		}

		Ok(())
	}

	pub async fn stop_workflow(&self, id: &str) -> Result<(), String> {
		let workflow = self.get_workflow(id).ok_or("Workflow not found.")?;
		if !self.is_running(id) {
			return Err("Workflow is not running.".to_string())
		}

		let workflow_future = Python::with_gil(|py| {
			let querent_stop_workflow = py
				.import(workflow.python_import_path.as_str())
				.map_err(|_| "Failed to import workflow.")?;
			let inner_fut = pyo3_asyncio::async_std::into_future(
				querent_stop_workflow
					.call_method0("stop")
					.map_err(|_| "Failed to stop workflow.")?,
			);
			if let Ok(fut) = inner_fut {
				Ok(fut)
			} else {
				Err("Failed to stop workflow.".to_string())
			}
		});
		match workflow_future {
			Ok(fut) => {
				let mut running_workflows = self.running_workflows.lock().unwrap();
				running_workflows.remove(id);
				fut.await.map_err(|_| "Failed to stop workflow.".to_string())?;
				Ok(())
			},
			Err(_) => return Err("Failed to stop workflow.".to_string()),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use async_std::task; // Import async-std for async testing

	#[test]
	fn test_start_workflow() {
		task::block_on(async {
			let workflow_manager = WorkflowManager::new();
			let workflow = Workflow {
				name: "Test Workflow".to_string(),
				id: "test_workflow".to_string(),
				python_import_path: "test_workflow".to_string(),
				python_start_function: "start".to_string(),
				config: Config::default(),
			};
			workflow_manager.add_workflow(workflow.clone()).unwrap();
			let res = workflow_manager.start_workflow(&workflow.id).await;
			match res {
				Ok(_) => assert!(workflow_manager.is_running("test_workflow")),
				Err(_) => assert!(false),
			}
		});
	}
}
