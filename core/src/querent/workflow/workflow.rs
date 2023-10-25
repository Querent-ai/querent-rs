use crate::config::Config;
use pyo3::prelude::*;
use std::{collections::HashMap, sync::Mutex};

#[derive(Debug, Clone)]
pub struct Workflow {
	pub name: String,
	pub id: String,
	pub config: Config,
}

#[derive(Debug)]
pub struct WorkflowManager {
	workflows: Mutex<HashMap<String, Workflow>>,
}

impl WorkflowManager {
	pub fn new() -> Self {
		WorkflowManager { workflows: Mutex::new(HashMap::new()) }
	}

	pub fn add_workflow(&self, workflow: Workflow) -> Result<(), String> {
		let mut workflows = self.workflows.lock().map_err(|_| "Mutex lock failed.".to_string())?;
		if workflows.contains_key(&workflow.id) {
			Err("Workflow with the same ID already exists.".to_string())
		} else {
			workflows.insert(workflow.id.clone(), workflow);
			Ok(())
		}
	}

	pub fn remove_workflow(&self, id: &str) -> Result<(), String> {
		let mut workflows = self.workflows.lock().map_err(|_| "Mutex lock failed.".to_string())?;
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

	pub fn start_workflow(&self, id: &str) -> Result<(), String> {
		let workflow = self.get_workflow(id).ok_or("Workflow not found.")?;
		Python::with_gil(|py| {
			// Implement Python interaction with the workflow here
			// For example: let workflow_config = &workflow.config;
			Ok(())
		})
	}

	pub fn kill_workflow(&self, id: &str) -> Result<(), String> {
		let workflow = self.get_workflow(id).ok_or("Workflow not found.")?;
		Python::with_gil(|py| {
			// Implement Python interaction with the workflow here
			// For example: let workflow_config = &workflow.config;
			Ok(())
		})
	}

	pub fn restart_workflow(&self, id: &str) -> Result<(), String> {
		let workflow = self.get_workflow(id).ok_or("Workflow not found.")?;
		Python::with_gil(|py| {
			// Implement Python interaction with the workflow here
			// For example: let workflow_config = &workflow.config;
			Ok(())
		})
	}

	fn check_workflow_exists(&self, workflow_id: &str) -> bool {
		self.workflows.lock().is_ok() && self.workflows.lock().unwrap().contains_key(workflow_id)
	}
}
