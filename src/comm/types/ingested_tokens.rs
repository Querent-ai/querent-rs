use pyo3::{prelude::*, types::PyDict, PyObject, Python};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, utoipa::ToSchema)]
pub struct IngestedTokens {
	pub data: Option<Vec<String>>,
	pub file: String,
	pub is_token_stream: Option<bool>,
}

impl IntoPy<PyObject> for IngestedTokens {
	fn into_py(self, py: Python) -> PyObject {
		let token_dict = PyDict::new(py);
		token_dict.set_item("data", &self.data).unwrap();
		token_dict.set_item("file", &self.file).unwrap();
		token_dict.set_item("is_token_stream", &self.is_token_stream).unwrap();

		token_dict.into()
	}
}

impl ToPyObject for IngestedTokens {
	fn to_object(&self, py: Python) -> PyObject {
		let token_dict = PyDict::new(py);
		token_dict.set_item("data", &self.data).unwrap();
		token_dict.set_item("file", &self.file).unwrap();
		token_dict.set_item("is_token_stream", &self.is_token_stream).unwrap();

		token_dict.into()
	}
}
