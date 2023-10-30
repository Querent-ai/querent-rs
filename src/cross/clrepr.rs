use crate::cross::clrepr_python::PythonRef;
use std::collections::{
	hash_map::{IntoIter, Iter, Keys},
	HashMap,
};

#[derive(Clone)]
pub struct CLReprObject(pub(crate) HashMap<String, CLRepr>);

impl CLReprObject {
	pub fn new() -> Self {
		Self(HashMap::new())
	}

	pub fn get(&self, key: &str) -> Option<&CLRepr> {
		self.0.get(key)
	}

	pub fn insert(&mut self, key: String, value: CLRepr) -> Option<CLRepr> {
		self.0.insert(key, value)
	}

	pub fn into_iter(self) -> IntoIter<String, CLRepr> {
		self.0.into_iter()
	}

	pub fn iter(&self) -> Iter<String, CLRepr> {
		self.0.iter()
	}

	pub fn keys(&self) -> Keys<'_, String, CLRepr> {
		self.0.keys()
	}
}

impl std::fmt::Debug for CLReprObject {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Debug::fmt(&self.0, f)
	}
}

impl std::fmt::Display for CLReprObject {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Debug::fmt(&self.0, f)
	}
}

#[derive(Debug)]
pub enum CLReprKind {
	String,
	Bool,
	Float,
	Int,
	Tuple,
	Array,
	Object,
	JsFunction,
	PythonRef,
	Null,
}

#[derive(Debug, Clone)]
pub enum StringType {
	Normal,
	#[allow(unused)]
	Safe,
}

/// Cross language representation is abstraction to transfer values between
/// JavaScript and Python across Rust. Converting between two different languages requires
/// to use Context which is available on the call (one for python and one for js), which result as
/// blocking.
#[derive(Debug, Clone)]
pub enum CLRepr {
	String(String, StringType),
	Bool(bool),
	Float(f64),
	Int(i64),
	#[allow(dead_code)]
	Tuple(Vec<CLRepr>),
	Array(Vec<CLRepr>),
	Object(CLReprObject),
	PythonRef(PythonRef),
	Null,
}

impl std::fmt::Display for CLRepr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(&self, f)
	}
}

impl CLRepr {
	pub fn is_null(&self) -> bool {
		matches!(self, CLRepr::Null)
	}

	pub fn downcast_to_object(self) -> CLReprObject {
		match self {
			CLRepr::Object(obj) => obj,
			_ => panic!("downcast_to_object rejected, actual: {:?}", self.kind()),
		}
	}

	#[allow(unused)]
	pub fn kind(&self) -> CLReprKind {
		match self {
			CLRepr::String(_, _) => CLReprKind::String,
			CLRepr::Bool(_) => CLReprKind::Bool,
			CLRepr::Float(_) => CLReprKind::Float,
			CLRepr::Int(_) => CLReprKind::Int,
			CLRepr::Tuple(_) => CLReprKind::Tuple,
			CLRepr::Array(_) => CLReprKind::Array,
			CLRepr::Object(_) => CLReprKind::Object,
			CLRepr::PythonRef(_) => CLReprKind::PythonRef,
			CLRepr::Null => CLReprKind::Null,
		}
	}
}
