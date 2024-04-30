use pyo3::{prelude::*, types::PyDict, PyObject, Python};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, utoipa::ToSchema)]
pub struct IngestedTokens {
	pub data: Option<Vec<String>>,
	pub file: String,
	pub is_token_stream: Option<bool>,
	pub doc_source: String,
}

impl IntoPy<PyObject> for IngestedTokens {
	fn into_py(self, py: Python) -> PyObject {
		let token_dict = PyDict::new(py);
		token_dict.set_item("data", &self.data).unwrap();
		token_dict.set_item("file", &self.file).unwrap();
		token_dict.set_item("is_token_stream", &self.is_token_stream).unwrap();
		token_dict.set_item("doc_source", &self.doc_source).unwrap();

		token_dict.into()
	}
}

impl FromPyObject<'_> for IngestedTokens {
	fn extract(ob: &PyAny) -> PyResult<Self> {
		let data = ob.get_item("data")?.extract()?;
		let file = ob.get_item("file")?.extract()?;
		let is_token_stream = ob.get_item("is_token_stream")?.extract()?;
		let doc_source = ob.get_item("doc_source")?.extract()?;

		Ok(IngestedTokens { data, file, is_token_stream, doc_source })
	}
}

impl ToPyObject for IngestedTokens {
	fn to_object(&self, py: Python) -> PyObject {
		let token_dict = PyDict::new(py);
		token_dict.set_item("data", &self.data).unwrap();
		token_dict.set_item("file", &self.file).unwrap();
		token_dict.set_item("is_token_stream", &self.is_token_stream).unwrap();
		token_dict.set_item("doc_source", &self.doc_source).unwrap();

		token_dict.into()
	}
}
