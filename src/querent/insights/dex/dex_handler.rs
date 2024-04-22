use pyo3::{prelude::*, types::PyDict};
use serde::{Deserialize, Serialize};

use super::knowledge::{PaginatedDiscoveredKnowledge, PaginatedSemanticKnowledge};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaginatedSemanticKnowledgeRequest {
	pub request_id: String,
	pub page: i32,
	pub page_size: i32,
	pub collection_id: Option<String>,
}

impl<'a> FromPyObject<'a> for PaginatedSemanticKnowledgeRequest {
	fn extract(ob: &'a PyAny) -> PyResult<Self> {
		let request_id = ob.get_item("request_id")?.extract()?;
		let page = ob.get_item("page")?.extract()?;
		let page_size = ob.get_item("page_size")?.extract()?;
		let collection_id = ob.get_item("collection_id")?.extract()?;

		Ok(PaginatedSemanticKnowledgeRequest { request_id, page, page_size, collection_id })
	}
}

impl IntoPy<PyObject> for PaginatedSemanticKnowledgeRequest {
	fn into_py(self, py: Python) -> PyObject {
		let request_dict = PyDict::new(py);
		request_dict.set_item("request_id", &self.request_id).unwrap();
		request_dict.set_item("page", &self.page).unwrap();
		request_dict.set_item("page_size", &self.page_size).unwrap();
		request_dict.set_item("collection_id", &self.collection_id).unwrap();

		request_dict.into()
	}
}

impl ToPyObject for PaginatedSemanticKnowledgeRequest {
	fn to_object(&self, py: Python) -> PyObject {
		let request_dict = PyDict::new(py);
		request_dict.set_item("request_id", &self.request_id).unwrap();
		request_dict.set_item("page", &self.page).unwrap();
		request_dict.set_item("page_size", &self.page_size).unwrap();
		request_dict.set_item("collection_id", &self.collection_id).unwrap();

		request_dict.into()
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaginatedDiscoveredKnowledgeRequest {
	pub request_id: String,
	pub page: i32,
	pub page_size: i32,
	pub session_id: String,
}

impl<'a> FromPyObject<'a> for PaginatedDiscoveredKnowledgeRequest {
	fn extract(ob: &'a PyAny) -> PyResult<Self> {
		let request_id = ob.get_item("request_id")?.extract()?;
		let page = ob.get_item("page")?.extract()?;
		let page_size = ob.get_item("page_size")?.extract()?;
		let session_id = ob.get_item("session_id")?.extract()?;

		Ok(PaginatedDiscoveredKnowledgeRequest { request_id, page, page_size, session_id })
	}
}

impl IntoPy<PyObject> for PaginatedDiscoveredKnowledgeRequest {
	fn into_py(self, py: Python) -> PyObject {
		let request_dict = PyDict::new(py);
		request_dict.set_item("request_id", &self.request_id).unwrap();
		request_dict.set_item("page", &self.page).unwrap();
		request_dict.set_item("page_size", &self.page_size).unwrap();
		request_dict.set_item("session_id", &self.session_id).unwrap();

		request_dict.into()
	}
}

impl ToPyObject for PaginatedDiscoveredKnowledgeRequest {
	fn to_object(&self, py: Python) -> PyObject {
		let request_dict = PyDict::new(py);
		request_dict.set_item("request_id", &self.request_id).unwrap();
		request_dict.set_item("page", &self.page).unwrap();
		request_dict.set_item("page_size", &self.page_size).unwrap();
		request_dict.set_item("session_id", &self.session_id).unwrap();

		request_dict.into()
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DocumentRequest {
	pub document_id: String,
	pub document_extension: String,
	pub document_source: String,
	pub session_id: Option<String>,
}

impl<'a> FromPyObject<'a> for DocumentRequest {
	fn extract(ob: &'a PyAny) -> PyResult<Self> {
		let document_id = ob.get_item("document_id")?.extract()?;
		let document_extension = ob.get_item("document_extension")?.extract()?;
		let document_source = ob.get_item("document_source")?.extract()?;
		let session_id = ob.get_item("session_id")?.extract()?;

		Ok(DocumentRequest { document_id, document_extension, document_source, session_id })
	}
}

impl IntoPy<PyObject> for DocumentRequest {
	fn into_py(self, py: Python) -> PyObject {
		let document_dict = PyDict::new(py);
		document_dict.set_item("document_id", &self.document_id).unwrap();
		document_dict.set_item("document_extension", &self.document_extension).unwrap();
		document_dict.set_item("document_source", &self.document_source).unwrap();
		document_dict.set_item("session_id", &self.session_id).unwrap();

		document_dict.into()
	}
}

impl ToPyObject for DocumentRequest {
	fn to_object(&self, py: Python) -> PyObject {
		let document_dict = PyDict::new(py);
		document_dict.set_item("document_id", &self.document_id).unwrap();
		document_dict.set_item("document_extension", &self.document_extension).unwrap();
		document_dict.set_item("document_source", &self.document_source).unwrap();
		document_dict.set_item("session_id", &self.session_id).unwrap();

		document_dict.into()
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DocumentResponse {
	pub document_id: String,
	pub document_bytes: Vec<u8>,
	pub document_extension: String,
	pub document_source: String,
}

impl<'a> FromPyObject<'a> for DocumentResponse {
	fn extract(ob: &'a PyAny) -> PyResult<Self> {
		let document_id = ob.get_item("document_id")?.extract()?;
		let document_bytes = ob.get_item("document_bytes")?.extract()?;
		let document_extension = ob.get_item("document_extension")?.extract()?;
		let document_source = ob.get_item("document_source")?.extract()?;

		Ok(DocumentResponse { document_id, document_bytes, document_extension, document_source })
	}
}

impl IntoPy<PyObject> for DocumentResponse {
	fn into_py(self, py: Python) -> PyObject {
		let document_dict = PyDict::new(py);
		document_dict.set_item("document_id", &self.document_id).unwrap();
		document_dict.set_item("document_bytes", &self.document_bytes).unwrap();
		document_dict.set_item("document_extension", &self.document_extension).unwrap();
		document_dict.set_item("document_source", &self.document_source).unwrap();

		document_dict.into()
	}
}

impl ToPyObject for DocumentResponse {
	fn to_object(&self, py: Python) -> PyObject {
		let document_dict = PyDict::new(py);
		document_dict.set_item("document_id", &self.document_id).unwrap();
		document_dict.set_item("document_bytes", &self.document_bytes).unwrap();
		document_dict.set_item("document_extension", &self.document_extension).unwrap();
		document_dict.set_item("document_source", &self.document_source).unwrap();

		document_dict.into()
	}
}

// Define the base interface for data exchange between python and rust
pub trait DexInterface {
	/// Request a paginated knowledge response
	fn request_paginated_knowledge(
		&mut self,
		request: PaginatedSemanticKnowledgeRequest,
	) -> Option<PaginatedSemanticKnowledge>;

	/// Request a paginated discovered knowledge response
	fn request_paginated_discovered_knowledge(
		&mut self,
		request: PaginatedDiscoveredKnowledgeRequest,
	) -> Option<PaginatedDiscoveredKnowledge>;

	/// Request a document response
	fn request_documents(
		&mut self,
		documents: Vec<DocumentRequest>,
	) -> Option<Vec<DocumentResponse>>;

	/// Request all documents given for discovery session
	fn request_all_documents(&mut self, session_id: String) -> Option<Vec<DocumentResponse>>;
}

// Define a basic event handler struct
#[derive(Clone, Debug)]
#[pyclass]
pub struct DataTransferLayer {
	pub knowledge_sender: Option<crossbeam_channel::Sender<PaginatedSemanticKnowledgeRequest>>,
	pub knowledge_receiver: Option<crossbeam_channel::Receiver<PaginatedSemanticKnowledge>>,
	pub discovered_knowledge_sender:
		Option<crossbeam_channel::Sender<PaginatedDiscoveredKnowledgeRequest>>,
	pub discovered_knowledge_receiver:
		Option<crossbeam_channel::Receiver<PaginatedDiscoveredKnowledge>>,
	pub document_sender: Option<crossbeam_channel::Sender<Vec<DocumentRequest>>>,
	pub document_receiver: Option<crossbeam_channel::Receiver<Vec<DocumentResponse>>>,
}

impl DataTransferLayer {
	// Constructor for EventHandler
	pub fn new(
		knowledge_sender: Option<crossbeam_channel::Sender<PaginatedSemanticKnowledgeRequest>>,
		knowledge_receiver: Option<crossbeam_channel::Receiver<PaginatedSemanticKnowledge>>,
		discovered_knowledge_sender: Option<
			crossbeam_channel::Sender<PaginatedDiscoveredKnowledgeRequest>,
		>,
		discovered_knowledge_receiver: Option<
			crossbeam_channel::Receiver<PaginatedDiscoveredKnowledge>,
		>,
		document_sender: Option<crossbeam_channel::Sender<Vec<DocumentRequest>>>,
		document_receiver: Option<crossbeam_channel::Receiver<Vec<DocumentResponse>>>,
	) -> Self {
		DataTransferLayer {
			knowledge_sender,
			knowledge_receiver,
			discovered_knowledge_sender,
			discovered_knowledge_receiver,
			document_sender,
			document_receiver,
		}
	}
}

// Define a Python-compatible message
#[derive(Clone, Debug)]
#[pyclass]
pub struct PyDexInterface {
	dex_handler: DataTransferLayer,
}

#[pymethods]
impl PyDexInterface {
	// Python constructor for PyMessageInterface
	#[new]
	pub fn new(dex_handler: DataTransferLayer) -> Self {
		PyDexInterface { dex_handler }
	}

	// Request a paginated knowledge response
	pub fn request_paginated_knowledge(
		&mut self,
		request: PaginatedSemanticKnowledgeRequest,
	) -> Option<PaginatedSemanticKnowledge> {
		self.dex_handler.request_paginated_knowledge(request)
	}

	// Request a paginated discovered knowledge response
	pub fn request_paginated_discovered_knowledge(
		&mut self,
		request: PaginatedDiscoveredKnowledgeRequest,
	) -> Option<PaginatedDiscoveredKnowledge> {
		self.dex_handler.request_paginated_discovered_knowledge(request)
	}

	// Request a document response
	pub fn request_documents(
		&mut self,
		documents: Vec<DocumentRequest>,
	) -> Option<Vec<DocumentResponse>> {
		self.dex_handler.request_documents(documents)
	}
}

impl DexInterface for DataTransferLayer {
	fn request_paginated_knowledge(
		&mut self,
		request: PaginatedSemanticKnowledgeRequest,
	) -> Option<PaginatedSemanticKnowledge> {
		self.knowledge_sender.as_ref()?.send(request).ok()?;

		self.knowledge_receiver.as_ref()?.recv().ok()
	}

	fn request_paginated_discovered_knowledge(
		&mut self,
		request: PaginatedDiscoveredKnowledgeRequest,
	) -> Option<PaginatedDiscoveredKnowledge> {
		self.discovered_knowledge_sender.as_ref()?.send(request).ok()?;

		self.discovered_knowledge_receiver.as_ref()?.recv().ok()
	}

	fn request_documents(
		&mut self,
		documents: Vec<DocumentRequest>,
	) -> Option<Vec<DocumentResponse>> {
		self.document_sender.as_ref()?.send(documents).ok()?;

		self.document_receiver.as_ref()?.recv().ok()
	}

	fn request_all_documents(&mut self, session_id: String) -> Option<Vec<DocumentResponse>> {
		let request = vec![DocumentRequest {
			document_id: "".to_string(),
			document_extension: "".to_string(),
			document_source: "".to_string(),
			session_id: Some(session_id),
		}];
		self.request_documents(request)
	}
}
