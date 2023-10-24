use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Config {
	pub version: f32,
	pub querent_id: String,
	pub querent_name: String,
	pub workflow: WorkflowConfig,
	pub collectors: Vec<CollectorConfig>,
	pub engines: Vec<EngineConfig>,
	pub resource: Option<ResourceConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowConfig {
	name: String,
	id: String,
	config: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectorConfig {
	name: String,
	backend: String,
	config: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EngineConfig {
	name: String,
	num_workers: Option<u32>,
	max_retries: Option<u32>,
	retry_interval: Option<u32>,
	message_throttle_limit: Option<u32>,
	message_throttle_delay: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceConfig {
	max_workers_allowed: Option<u32>,
	max_workers_per_collector: Option<u32>,
	max_workers_per_engine: Option<u32>,
	max_workers_per_querent: Option<u32>,
}
