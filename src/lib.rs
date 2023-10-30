//! Querent Rust Bridge
//! Rust bridge for querent async RDF Knowledge Graph python library
//!  - Providers a Rust async interface to the querent python library
//!  - Interface ties to python module via pyo3
//!
//! # Example
//! use qurent_rs::Querent;

use once_cell::sync::OnceCell;
use querent::errors::QuerentError;
use tokio::runtime::{Builder, Runtime};

pub mod callbacks;
pub mod config;
pub mod cross;
pub mod querent;
pub mod util;

#[cfg(test)]
mod tests;

pub fn tokio_runtime() -> Result<&'static Runtime, QuerentError> {
	static RUNTIME: OnceCell<Runtime> = OnceCell::new();

	RUNTIME.get_or_try_init(|| {
		Builder::new_multi_thread()
			.enable_all()
			.build()
			.map_err(|err| QuerentError::internal(err.to_string()))
	})
}
