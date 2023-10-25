use crate::config::{config::WorkflowConfig, Config};
use pyo3::{prelude::*, types::PyDict, PyObject, ToPyObject};

#[derive(Debug)]
pub struct Workflow {
	pub name: String,
	pub id: String,
	pub config: Config,
}

#[derive(Debug)]
pub struct WorkflowManager {
	workflows: Vec<Workflow>,
}

impl WorkflowManager {
	pub fn new() -> Self {
		WorkflowManager { workflows: Vec::new() }
	}

	pub fn add_workflow(&mut self, workflow: Workflow) -> bool {
		if self.check_workflow_exists(&workflow.id) {
			return false
		}
		self.workflows.push(workflow);
		true
	}
	fn check_workflow_exists(&self, workflow_id: &str) -> bool {
		for workflow in &self.workflows {
			if workflow.id == workflow_id {
				return true
			}
		}
		false
	}

	pub fn start_workflow(&mut self, workflow_index: usize) {
		let workflow = &self.workflows[workflow_index];
		Python::with_gil(|py| {
			let workflow_dict = workflow.config.to_object(py);
		});
	}

	pub fn kill_workflow(&mut self, workflow_index: usize) {
		// Kill the Python workflow based on the given workflow_index
		// You can use Python interop to kill the workflow here.
		let workflow = &self.workflows[workflow_index];

		Python::with_gil(|py| {
			let workflow_dict = workflow.config.to_object(py);
		});
	}

	pub fn restart_workflow(&mut self, workflow_index: usize) {
		let workflow = &self.workflows[workflow_index];

		Python::with_gil(|py| {
			let workflow_dict = workflow.config.to_object(py);
		});
	}
}
