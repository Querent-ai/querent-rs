use futures::TryFutureExt;
use log;
use pyo3::{prelude::*, types::PyFunction};
use std::{collections::HashMap, sync::Mutex};
use tokio::runtime::Runtime;

use crate::{config::insight_config::InsightConfig, querent::{py_runtime, QuerentError}};

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
	/// Arguments to pass to the insight's start function.
	pub arguments: Vec<CLRepr>,
	/// Import function for the insight.
	pub import: String,
	/// Configuration for the insight.
	pub config: Option<InsightConfig>,
}

impl Insight {
	/// Runs the insight.
	pub async fn run(&self) ->  Result<(), QuerentError> {
		let runtime = py_runtime()?;
		let future = match &self.code {
			None => Python::with_gil(|py| {
				let async_mod = py.import("querent_insights").map_err(|e| {
					log::error!("Failed to import module {}: {}", "querent_insights", e);
					QuerentError::internal(e.to_string())
				})?;

				let coroutine =
					async_mod.getattr("run_insight").map_err(|_| {
						log::error!("Failed to find start function.");
						QuerentError::internal("Failed to find start function.".to_string())
					})?;

				let querent_py_fun: Py<PyFunction> = coroutine.extract().map_err(|e| {
					log::error!("Failed to extract function: {}", e);
					QuerentError::internal(e.to_string())
				})?;

				let mut config_pyobject: Option<PyObject> = None;
				if let Some(config) = &self.config {
					config_pyobject = Some(config.to_object(py));
				}
				let call_future =
					runtime.call_async(querent_py_fun, Vec::new(), config_pyobject);
				Ok(call_future)
			}),
			Some(code) => {
				let module_file: String = self.id + ".py";
				Python::with_gil(|py| {
					let dynamic_module = PyModule::from_code(
						py,
						code.as_str(),
						module_file.as_str(),
						self.name.as_str(),
					)
					.map_err(|e| {
						log::error!("Failed to import module {}: {}", code, e);
						QuerentError::internal(e.to_string())
					})?;

					let attr_fun =
						dynamic_module.getattr(_workflow.attr.as_str()).map_err(|_| {
							log::error!("Failed to find start function.");
							QuerentError::internal(
								"Failed to find start function.".to_string(),
							)
						})?;

					let querent_py_fun: Py<PyFunction> =
						attr_fun.extract().map_err(|e| {
							log::error!("Failed to extract function: {}", e);
							QuerentError::internal(e.to_string())
						})?;

					let mut config_pyobject: Option<PyObject> = None;
					if let Some(config) = &self.config {
						config_pyobject = Some(config.to_object(py));
					}
					let call_future =
						runtime.call_async(querent_py_fun, Vec::new(), config_pyobject);
					Ok(call_future)
				})
			},
		};
		Ok(())
	}
}