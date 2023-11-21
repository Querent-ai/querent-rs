use crate::{
	callbacks::PyEventCallbackInterface,
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
	event_callback: Option<PyEventCallbackInterface>,
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
			event_callback: None,
		}
	}

	/// Sets the name of the workflow.
	pub fn name(mut self, name: &str) -> Self {
		self.name = Some(name.to_string());
		self
	}

	/// Sets the import statement for the workflow.
	pub fn import(mut self, import: &str) -> Self {
		self.import = Some(import.to_string());
		self
	}

	/// Sets the attribute representing the start function of the workflow.
	pub fn attr(mut self, attr: &str) -> Self {
		self.attr = Some(attr.to_string());
		self
	}

	/// Sets the Python code for the workflow.
	pub fn code(mut self, code: &str) -> Self {
		self.code = Some(code.to_string());
		self
	}

	/// Adds an argument to the workflow.
	pub fn add_argument(mut self, argument: CLRepr) -> Self {
		self.arguments.push(argument);
		self
	}

	/// Sets the configuration for the workflow.
	pub fn config(mut self, config: Config) -> Self {
		self.config = Some(config);
		self
	}

	/// Sets the event callback for the workflow.
	pub fn event_callback(mut self, callback: PyEventCallbackInterface) -> Self {
		self.event_callback = Some(callback);
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
			event_callback: self.event_callback,
		}
	}
}
