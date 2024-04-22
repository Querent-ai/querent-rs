use futures::TryFutureExt;
use log;
use pyo3::{prelude::*, types::PyFunction};
use std::{collections::HashMap, sync::Mutex};
use tokio::runtime::Runtime;

use crate::config::insight_config::InsightConfig;

/// Represents a workflow.
#[derive(Debug, Clone)]
#[pyclass]
pub struct Insight {
	/// Name of the insight.
	pub name: String,
	/// Unique identifier for the insight.
	pub id: String,
	/// Unique id for insight job on python side
	pub insight_id: Option<String>,
	/// If insight_id is missing then this is the python code to execute
	pub code: Option<String>,
	/// Configuration for the insight.
	pub config: Option<InsightConfig>,
}
