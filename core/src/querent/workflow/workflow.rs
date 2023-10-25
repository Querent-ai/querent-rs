use std::collections::HashMap;

use crate::config::{config::WorkflowConfig, Config};
use dryoc::classic::crypto_shorthash::Hash;
use pyo3::{prelude::*, types::PyDict, PyObject, ToPyObject};

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
	fn check_workflow_exists(&self, workflow_id: &str) -> bool {
		self.workflows.contains_key(workflow_id)
	}

	pub fn start_workflow(&mut self, id: &str) {
		let workflow = &self.workflows[id];
		Python::with_gil(|py| {
			let workflow_config = &workflow.config;
		});
	}

	pub fn kill_workflow(&mut self, id: &str) {
		let workflow = &self.workflows[id];

		Python::with_gil(|py| {
			let workflow_config = &workflow.config;
		});
	}

	pub fn restart_workflow(&mut self, id: &str) {
		let workflow = &self.workflows[id];

		Python::with_gil(|py| {
			let workflow_config = &workflow.config;
		});
	}
}
