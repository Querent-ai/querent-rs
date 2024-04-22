#![allow(unused_imports)]
pub mod errors;
pub use errors::*;
pub mod workflow;
pub use workflow::*;
pub use workflow_builder::*;
pub mod querent;
pub use querent::*;
pub mod py_runtime;
pub use py_runtime::*;
pub mod py_process;
pub use py_process::*;
pub mod insights;

use pyo3::{PyObject, Python};
/// Generic config trait for Input type C in PyAsyncFun
/// This trait is used to pass configuration to the python function
/// The configuration is passed as a python object
pub trait ConfigTrait {
	fn to_object(&self, py: Python) -> PyObject;
}
