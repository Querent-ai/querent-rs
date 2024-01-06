use std::collections::HashMap;

use tokio::sync::mpsc;

use crate::{
	callbacks::{interface::EventHandler, EventState, EventType},
	comm::ChannelHandler,
};

use super::{
	config::{CollectorConfig, EngineConfig, ResourceConfig, WorkflowConfig},
	Config,
};

/// Builder for constructing a `Config`.
#[derive(Default)]
pub struct ConfigBuilder {
	version: Option<f32>,
	querent_id: Option<String>,
	querent_name: Option<String>,
	workflow: Option<WorkflowConfig>,
	collectors: Option<Vec<CollectorConfig>>,
	engines: Option<Vec<EngineConfig>>,
	resource: Option<Option<ResourceConfig>>,
	event_handler: Option<EventHandler>,
	channel_handler: Option<ChannelHandler>,
	event_sender: Option<mpsc::Sender<(EventType, EventState)>>,
}

impl ConfigBuilder {
	/// Creates a new `ConfigBuilder`.
	pub fn new() -> Self {
		Default::default()
	}

	/// Sets the version for the `Config`.
	pub fn version(mut self, version: f32) -> Self {
		self.version = Some(version);
		self
	}

	/// Sets the querent ID for the `Config`.
	pub fn querent_id(mut self, querent_id: String) -> Self {
		self.querent_id = Some(querent_id);
		self
	}

	/// Sets the querent name for the `Config`.
	pub fn querent_name(mut self, querent_name: String) -> Self {
		self.querent_name = Some(querent_name);
		self
	}

	/// Sets the workflow for the `Config`.
	pub fn workflow(mut self, workflow: WorkflowConfig) -> Self {
		self.workflow = Some(workflow);
		self
	}

	/// Sets the collectors for the `Config`.
	pub fn collectors(mut self, collectors: Vec<CollectorConfig>) -> Self {
		self.collectors = Some(collectors);
		self
	}

	/// Sets the engines for the `Config`.
	pub fn engines(mut self, engines: Vec<EngineConfig>) -> Self {
		self.engines = Some(engines);
		self
	}

	/// Sets the resource for the `Config`.
	pub fn resource(mut self, resource: Option<ResourceConfig>) -> Self {
		self.resource = Some(resource);
		self
	}

	/// Sets the event sender for the `Config`.
	pub fn event_sender(mut self, event_sender: mpsc::Sender<(EventType, EventState)>) -> Self {
		self.event_sender = Some(event_sender);
		self
	}

	/// Sets the event handler for the `Config`.
	pub fn event_handler(mut self, event_handler: EventHandler) -> Self {
		self.event_handler = Some(event_handler);
		self
	}

	/// Sets the channel handler for the `Config`.
	pub fn channel_handler(mut self, channel_handler: ChannelHandler) -> Self {
		self.channel_handler = Some(channel_handler);
		self
	}

	/// Builds the `Config` using the configured parameters.
	pub fn build(self) -> Config {
		Config {
			version: self.version.unwrap_or_else(|| 0.1),
			querent_id: self.querent_id.unwrap_or_else(|| "querent".to_string()),
			querent_name: self.querent_name.unwrap_or_else(|| "Querent".to_string()),
			workflow: self.workflow.unwrap_or_else(|| WorkflowConfig {
				name: "workflow".to_string(),
				id: "workflow".to_string(),
				config: HashMap::new(),
				channel: None,
				inner_channel: None,
				inner_event_handler: Some(EventHandler::new(self.event_sender)),
				event_handler: None,
			}),
			collectors: self.collectors.unwrap_or_else(Vec::new),
			engines: self.engines.unwrap_or_else(Vec::new),
			resource: self.resource.unwrap_or_else(|| None),
		}
	}
}
