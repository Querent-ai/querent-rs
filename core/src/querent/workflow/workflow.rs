use std::collections::HashMap;

use crate::config::Config;
use pyo3::prelude::*;

#[derive(Debug)]
pub struct Workflow {
	pub name: String,
	pub id: String,
	pub config: Config,
}

#[derive(Debug)]
pub struct WorkflowManager {
	workflows: HashMap<String, Workflow>,
}

impl WorkflowManager {
	pub fn new() -> Self {
		WorkflowManager { workflows: HashMap::new() }
	}

	pub fn add_workflow(&mut self, workflow: Workflow) -> bool {
		if self.check_workflow_exists(&workflow.id) {
			return false
		}
		self.workflows.insert(workflow.id.clone(), workflow);
		true
	}

	pub fn remove_workflow(&mut self, id: &str) -> bool {
		if self.check_workflow_exists(id) {
			self.workflows.remove(id);
			return true
		}
		false
	}

	pub fn get_workflow(&self, id: &str) -> Option<&Workflow> {
		self.workflows.get(id)
	}

	pub fn get_workflows(&self) -> Vec<&Workflow> {
		self.workflows.values().collect()
	}

	pub fn start_workflow(&mut self, id: &str) {
		if let Some(workflow) = self.workflows.get(id) {
			Python::with_gil(|py| {
				let workflow_config = &workflow.config;
			});
		}
	}

	pub fn kill_workflow(&mut self, id: &str) {
		if let Some(workflow) = self.workflows.get(id) {
			Python::with_gil(|py| {
				let workflow_config = &workflow.config;
			});
		}
	}

	pub fn restart_workflow(&mut self, id: &str) {
		if let Some(workflow) = self.workflows.get(id) {
			Python::with_gil(|py| {
				let workflow_config = &workflow.config;
			});
		}
	}

	fn check_workflow_exists(&self, workflow_id: &str) -> bool {
		self.workflows.contains_key(workflow_id)
	}
}
