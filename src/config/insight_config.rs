use std::collections::HashMap;

use pyo3::{prelude::*, types::PyDict, PyObject, ToPyObject};

use crate::querent::insights::dex::dex_handler::{DataTransferLayer, PyDexInterface};

/// Configuration struct representing the overall setup for a insight flow.
/// A typical insight config will have the following
#[derive(Debug, Clone)]
#[pyclass]
pub struct InsightConfig {
	/// A unique identifier for the insight job
	pub id: String,
	/// Unique identifier for the insight selected
	pub insight_id: String,
	/// The discovery session id for which this insight job should be run
	pub discovery_session_id: String,
	/// Inner data exchange layer or dex
	inner_dex: Option<DataTransferLayer>,
	/// PyObject for the dex handler
	#[pyo3(get, set)]
	pub dex: Option<PyObject>,
	/// Any custom python code to execute by the insight
	pub custom_code: Option<String>,
	/// Specific insight related data that needs to be sent to the insight
	pub additional_inputs: HashMap<String, String>,
}

impl ToPyObject for InsightConfig {
	/// Converts a InsightConfig to a Python object.
	fn to_object(&self, py: Python) -> PyObject {
		let config_dict = PyDict::new(py);
		config_dict.set_item("id", &self.id).unwrap();
		config_dict.set_item("insight_id", &self.insight_id).unwrap();
		config_dict
			.set_item("discovery_session_id", &self.discovery_session_id)
			.unwrap();
		if let Some(inner_dex) = &self.inner_dex {
			let dex_interface = PyDexInterface::new(inner_dex.clone());
			let dex: PyObject =
				Py::new(py, dex_interface).expect("Unable to create class").into_py(py);
			config_dict.set_item("dex", dex).unwrap();
		}
		if let Some(custom_code) = &self.custom_code {
			config_dict.set_item("custom_code", custom_code).unwrap();
		}
		if !self.additional_inputs.is_empty() {
			let additional_input_dict = PyDict::new(py);
			for (key, value) in &self.additional_inputs {
				additional_input_dict.set_item(key, value).unwrap();
			}
			config_dict.set_item("additional_inputs", additional_input_dict).unwrap();
		}
		config_dict.to_object(py)
	}
}
