use pyo3::{prelude::*, types::PyDict, PyObject, Python};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmbeddedKnowledge {
	pub document_id: String,
	pub document_source: String,
	pub knowledge: String,
	pub embeddings: Option<Vec<f32>>,
	pub predicate: String,
	pub sentence: Option<String>,
	pub collection_id: Option<String>,
}

impl IntoPy<PyObject> for EmbeddedKnowledge {
	fn into_py(self, py: Python) -> PyObject {
		let knowledge_dict = PyDict::new(py);
		knowledge_dict.set_item("document_id", &self.document_id).unwrap();
		knowledge_dict.set_item("document_source", &self.document_source).unwrap();
		knowledge_dict.set_item("knowledge", &self.knowledge).unwrap();
		knowledge_dict.set_item("embeddings", &self.embeddings).unwrap();
		knowledge_dict.set_item("predicate", &self.predicate).unwrap();
		knowledge_dict.set_item("sentence", &self.sentence).unwrap();
		knowledge_dict.set_item("collection_id", &self.collection_id).unwrap();

		knowledge_dict.into()
	}
}

impl ToPyObject for EmbeddedKnowledge {
	fn to_object(&self, py: Python) -> PyObject {
		let knowledge_dict = PyDict::new(py);
		knowledge_dict.set_item("document_id", &self.document_id).unwrap();
		knowledge_dict.set_item("document_source", &self.document_source).unwrap();
		knowledge_dict.set_item("knowledge", &self.knowledge).unwrap();
		knowledge_dict.set_item("embeddings", &self.embeddings).unwrap();
		knowledge_dict.set_item("predicate", &self.predicate).unwrap();
		knowledge_dict.set_item("sentence", &self.sentence).unwrap();
		knowledge_dict.set_item("collection_id", &self.collection_id).unwrap();

		knowledge_dict.into()
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiscoveredKnowledge {
	pub doc_id: String,
	pub doc_source: String,
	pub sentence: String,
	pub knowledge: String,
	pub subject: String,
	pub object: String,
	pub predicate: String,
	pub cosine_distance: Option<f64>,
	pub query_embedding: Option<Vec<f32>>,
	pub session_id: Option<String>,
}

impl IntoPy<PyObject> for DiscoveredKnowledge {
	fn into_py(self, py: Python) -> PyObject {
		let knowledge_dict = PyDict::new(py);
		knowledge_dict.set_item("doc_id", &self.doc_id).unwrap();
		knowledge_dict.set_item("doc_source", &self.doc_source).unwrap();
		knowledge_dict.set_item("sentence", &self.sentence).unwrap();
		knowledge_dict.set_item("knowledge", &self.knowledge).unwrap();
		knowledge_dict.set_item("subject", &self.subject).unwrap();
		knowledge_dict.set_item("object", &self.object).unwrap();
		knowledge_dict.set_item("predicate", &self.predicate).unwrap();
		knowledge_dict.set_item("cosine_distance", &self.cosine_distance).unwrap();
		knowledge_dict.set_item("query_embedding", &self.query_embedding).unwrap();
		knowledge_dict.set_item("session_id", &self.session_id).unwrap();

		knowledge_dict.into()
	}
}

impl ToPyObject for DiscoveredKnowledge {
	fn to_object(&self, py: Python) -> PyObject {
		let knowledge_dict = PyDict::new(py);
		knowledge_dict.set_item("doc_id", &self.doc_id).unwrap();
		knowledge_dict.set_item("doc_source", &self.doc_source).unwrap();
		knowledge_dict.set_item("sentence", &self.sentence).unwrap();
		knowledge_dict.set_item("knowledge", &self.knowledge).unwrap();
		knowledge_dict.set_item("subject", &self.subject).unwrap();
		knowledge_dict.set_item("object", &self.object).unwrap();
		knowledge_dict.set_item("predicate", &self.predicate).unwrap();
		knowledge_dict.set_item("cosine_distance", &self.cosine_distance).unwrap();
		knowledge_dict.set_item("query_embedding", &self.query_embedding).unwrap();
		knowledge_dict.set_item("session_id", &self.session_id).unwrap();

		knowledge_dict.into()
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaginatedSemanticKnowledge {
	pub total: i64,
	pub page: i64,
	pub per_page: i64,
	pub data: Vec<EmbeddedKnowledge>,
}

impl IntoPy<PyObject> for PaginatedSemanticKnowledge {
	fn into_py(self, py: Python) -> PyObject {
		let knowledge_dict = PyDict::new(py);
		knowledge_dict.set_item("total", &self.total).unwrap();
		knowledge_dict.set_item("page", &self.page).unwrap();
		knowledge_dict.set_item("per_page", &self.per_page).unwrap();
		knowledge_dict.set_item("data", &self.data).unwrap();

		knowledge_dict.into()
	}
}

impl ToPyObject for PaginatedSemanticKnowledge {
	fn to_object(&self, py: Python) -> PyObject {
		let knowledge_dict = PyDict::new(py);
		knowledge_dict.set_item("total", &self.total).unwrap();
		knowledge_dict.set_item("page", &self.page).unwrap();
		knowledge_dict.set_item("per_page", &self.per_page).unwrap();
		knowledge_dict.set_item("data", &self.data).unwrap();

		knowledge_dict.into()
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaginatedDiscoveredKnowledge {
	pub total: i64,
	pub page: i64,
	pub per_page: i64,
	pub data: Vec<DiscoveredKnowledge>,
}

impl IntoPy<PyObject> for PaginatedDiscoveredKnowledge {
	fn into_py(self, py: Python) -> PyObject {
		let knowledge_dict = PyDict::new(py);
		knowledge_dict.set_item("total", &self.total).unwrap();
		knowledge_dict.set_item("page", &self.page).unwrap();
		knowledge_dict.set_item("per_page", &self.per_page).unwrap();
		knowledge_dict.set_item("data", &self.data).unwrap();

		knowledge_dict.into()
	}
}

impl ToPyObject for PaginatedDiscoveredKnowledge {
	fn to_object(&self, py: Python) -> PyObject {
		let knowledge_dict = PyDict::new(py);
		knowledge_dict.set_item("total", &self.total).unwrap();
		knowledge_dict.set_item("page", &self.page).unwrap();
		knowledge_dict.set_item("per_page", &self.per_page).unwrap();
		knowledge_dict.set_item("data", &self.data).unwrap();

		knowledge_dict.into()
	}
}
