use std::{collections::HashMap, sync::Mutex};

use crate::config::Config;

use super::{QuerentError, Workflow, WorkflowManager};

/// Querent provides a high-level interface for working with workflows.
pub struct Querent {
	manager: WorkflowManager,
}

impl Querent {
	/// Creates a new Querent instance.
	pub fn new() -> Result<Self, String> {
		let manager = WorkflowManager::new()?;
		Ok(Self { manager })
	}

	/// Adds a workflow to Querent.
	pub fn add_workflow(&self, workflow: Workflow) -> Result<(), String> {
		self.manager.add_workflow(workflow)?;
		Ok(())
	}

	/// Starts all workflows asynchronously.
	pub async fn start_workflows(&self) -> Result<(), QuerentError> {
		self.manager.start_workflows().await?;
		Ok(())
	}

	/// Get all the workflows
	pub fn get_workflows(&self) -> Vec<Workflow> {
		self.manager.get_workflows()
	}
}
