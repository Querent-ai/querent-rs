use pyo3::{prelude::*, types::PyDict, PyObject, Python};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, utoipa::ToSchema)]
pub struct Data {
	pub data: Option<Vec<String>>,
	pub file: String,
	pub is_token_stream: Option<bool>,
	pub doc_source: String,
}