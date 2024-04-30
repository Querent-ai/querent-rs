use crate::{
	callbacks::PyEventCallbackInterface,
	comm::ChannelHandler,
	config::Config,
	cross::{CLRepr, StringType},
};

use super::Workflow;

/// Builder for constructing a `Workflow`.
pub struct WorkflowBuilder {
	name: Option<String>,
	id: String,
	import: Option<String>,
	attr: Option<String>,
	code: Option<String>,
	arguments: Vec<CLRepr>,
	config: Option<Config>,
}

impl WorkflowBuilder {
	/// Creates a new `WorkflowBuilder` with the required workflow ID.
	pub fn new(id: &str) -> Self {
		WorkflowBuilder {
			name: None,
			id: id.to_string(),
			import: None,
			attr: None,
			code: None,
			arguments: Vec::new(),
			config: None,
		}
	}

	/// Workflow from given workflow.
	pub fn from_workflow(workflow: Workflow) -> Self {
		WorkflowBuilder {
			name: Some(workflow.name),
			id: workflow.id,
			import: Some(workflow.import),
			attr: Some(workflow.attr),
			code: workflow.code,
			arguments: workflow.arguments,
			config: workflow.config,
		}
	}

	/// Sets the name of the workflow.
	pub fn name(mut self, name: &str) -> Self {
		self.name = Some(name.to_string());
		self
	}

	/// Sets the import statement for the workflow.
	pub fn import(mut self, import: Option<String>) -> Self {
		self.import = import;
		self
	}

	/// Sets the attribute representing the start function of the workflow.
	pub fn attr(mut self, attr: Option<String>) -> Self {
		self.attr = attr;
		self
	}

	/// Sets the Python code for the workflow.
	pub fn code(mut self, code: Option<String>) -> Self {
		self.code = code;
		self
	}
	/// add arguments to the workflow
	pub fn arguments(mut self, arguments: Vec<CLRepr>) -> Self {
		self.arguments = arguments;
		self
	}

	/// Sets the configuration for the workflow.
	pub fn config(mut self, config: Config) -> Self {
		self.config = Some(config);
		self
	}

	/// Builds the `Workflow` using the configured parameters.
	pub fn build(self) -> Workflow {
		Workflow {
			name: self.name.unwrap_or_else(|| self.id.clone()),
			id: self.id,
			import: self.import.unwrap_or_default(),
			attr: self.attr.unwrap_or_default(),
			code: self.code,
			arguments: self.arguments,
			config: self.config,
		}
	}
}
