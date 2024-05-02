//! Querent Rust Bridge
//! Rust bridge for querent async RDF Knowledge Graph python library
//!  - Providers a Rust async interface to the querent python library
//!  - Interface ties to python module via pyo3
//!
//! # Example
//! use qurent_rs::Querent;

use anyhow::Error;
use once_cell::sync::OnceCell;
use pyembed::MainPythonInterpreter;
use querent::{errors::QuerentError, Settings};
use tokio::runtime::{Builder, Runtime};
use tracing::info;

pub mod callbacks;
pub mod comm;
pub mod config;
pub mod cross;
pub mod querent;

#[cfg(test)]
mod tests;
pub mod util;
/// Python Standard Library ZIP
const PYTHON_STDLIB: &[u8] = include_bytes!("../pyembedded/stdlib.zip");
const PYTHON_VERSION: &'static str = include_str!("../pyembedded/VERSION");
const PIP_PYZ: &[u8] = include_bytes!("../pyembedded/pip.pyz");

/// Windows specific libraries that have to be unzipped
#[cfg(target_os = "windows")]
const PYTHON_LIBS: &[u8] = include_bytes!("../pyembedded/lib.zip");

/// Setup Python
pub fn setup() -> Result<(), Error> {
	let dir = Settings::get_folder()?;

	// Check python version
	let version_file = dir.join("PYTHON_VERSION");
	let version = match version_file.exists() {
		true => std::fs::read_to_string(&version_file)?,
		false => {
			std::fs::write(version_file, PYTHON_VERSION)?;
			String::new()
		},
	};
	// Write stdlib & pip
	if version != PYTHON_VERSION {
		info!("Writing python stdlib for {}", PYTHON_VERSION);
		std::fs::write(dir.join("python_stdlib.zip"), PYTHON_STDLIB)?;
		std::fs::write(dir.join("pip.pyz"), PIP_PYZ)?;
	}

	// Unzip Windows libs
	#[cfg(target_os = "windows")]
	{
		use std::io::Cursor;
		use zip::read::ZipArchive;
		let mut zip = ZipArchive::new(Cursor::new(PYTHON_LIBS))?;
		std::fs::create_dir_all(dir.join("lib"))?;
		zip.extract(dir.join("lib"))?;
	}

	Ok(())
}

pub fn base_python_interpreter() -> Result<(), Error> {
	// Setup python
	setup()?;
	let folder = Settings::get_folder()?;
	let mut config = querent::py_module::pyoxidizer_config(folder.clone())?;
	config
		.interpreter_config
		.module_search_paths
		.as_mut()
		.unwrap()
		.push(folder.join("pip.pyz"));
	let interpreter = MainPythonInterpreter::new(config)?;
	// Enable modern shell
	interpreter.with_gil(|py| {
		py.import("readline").ok();
	});
	Ok(())
}

pub mod busy_detector {
	use std::{
		sync::atomic::{AtomicBool, AtomicU64, Ordering},
		time::Instant,
	};

	use once_cell::sync::Lazy;
	use tracing::debug;
	static TIME_REF: Lazy<Instant> = Lazy::new(Instant::now);
	static ENABLED: AtomicBool = AtomicBool::new(false);

	const ALLOWED_DELAY_MICROS: u64 = 5000;
	const DEBUG_SUPPRESSION_MICROS: u64 = 30_000_000;

	thread_local!(static LAST_UNPARK_TIMESTAMP: AtomicU64 = AtomicU64::new(0));
	static NEXT_DEBUG_TIMESTAMP: AtomicU64 = AtomicU64::new(0);
	static SUPPRESSED_DEBUG_COUNT: AtomicU64 = AtomicU64::new(0);

	pub fn set_enabled(enabled: bool) {
		ENABLED.store(enabled, Ordering::Relaxed);
	}

	pub fn thread_unpark() {
		LAST_UNPARK_TIMESTAMP.with(|time| {
			let now = Instant::now().checked_duration_since(*TIME_REF).unwrap_or_default();
			time.store(now.as_micros() as u64, Ordering::Relaxed);
		})
	}

	pub fn thread_park() {
		if !ENABLED.load(Ordering::Relaxed) {
			return;
		}

		LAST_UNPARK_TIMESTAMP.with(|time| {
			let now = Instant::now().checked_duration_since(*TIME_REF).unwrap_or_default();
			let now = now.as_micros() as u64;
			let delta = now - time.load(Ordering::Relaxed);
			if delta > ALLOWED_DELAY_MICROS {
				emit_debug(delta, now);
			}
		})
	}

	fn emit_debug(delta: u64, now: u64) {
		if NEXT_DEBUG_TIMESTAMP
			.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |next_debug| {
				if next_debug < now {
					Some(now + DEBUG_SUPPRESSION_MICROS)
				} else {
					None
				}
			})
			.is_err()
		{
			// a debug was emited recently, don't emit log for this one
			SUPPRESSED_DEBUG_COUNT.fetch_add(1, Ordering::Relaxed);
			return;
		}

		let suppressed = SUPPRESSED_DEBUG_COUNT.swap(0, Ordering::Relaxed);
		if suppressed == 0 {
			debug!("thread wasn't parked for {delta}µs, is the runtime too busy?");
		} else {
			debug!(
				"thread wasn't parked for {delta}µs, is the runtime too busy? ({suppressed} \
                 similar messages suppressed)"
			);
		}
	}
}

pub fn tokio_runtime() -> Result<&'static Runtime, QuerentError> {
	static RUNTIME: OnceCell<Runtime> = OnceCell::new();

	RUNTIME.get_or_try_init(|| {
		Builder::new_multi_thread()
			.enable_all()
			.on_thread_unpark(busy_detector::thread_unpark)
			.on_thread_park(busy_detector::thread_park)
			.build()
			.map_err(|err| QuerentError::internal(err.to_string()))
	})
}
