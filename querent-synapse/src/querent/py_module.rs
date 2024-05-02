use std::{
	fs::File,
	io::{BufReader, BufWriter, Write},
	path::{Path, PathBuf},
};

use anyhow::{anyhow, Error};
use directories::ProjectDirs;
use pyembed::OxidizedPythonInterpreterConfig;
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
	pub version: Option<i32>,
}

impl Settings {
	/// Get app data folder
	pub fn get_folder() -> Result<PathBuf, Error> {
		// Android data dir override
		#[cfg(target_os = "android")]
		if let Ok(dir) = std::env::var("__ANDROID_DATA_DIR") {
			return Ok(PathBuf::from(dir));
		}

		let root =
			ProjectDirs::from("com", "Querent", "Querent").ok_or(anyhow!("Error getting dir!"))?;
		if !root.preference_dir().exists() {
			std::fs::create_dir_all(root.preference_dir())?;
		}
		Ok(root.preference_dir().to_owned())
	}
}

/// Get pyoxidizer config
pub fn pyoxidizer_config<'a>(
	dir: impl AsRef<Path>,
) -> Result<OxidizedPythonInterpreterConfig<'a>, Error> {
	mod pyoxidizer_config {
		include!("../../pyembedded/config.rs");
	}
	let folder = Settings::get_folder()?;

	let mut config = pyoxidizer_config::default_python_config();
	config.interpreter_config.filesystem_encoding = Some("utf-8".to_string());
	config.tcl_library = None;
	config.packed_resources = vec![];
	config.filesystem_importer = true;
	config.oxidized_importer = false;
	config.interpreter_config.isolated = Some(true);
	config.interpreter_config.use_environment = Some(false);
	config.interpreter_config.home = Some(dunce::canonicalize(dir)?);
	config.interpreter_config.module_search_paths = Some(vec![
		dunce::canonicalize(folder.join("python_stdlib.zip"))?,
		dunce::canonicalize(folder.join("pip.pyz"))?,
	]);
	#[cfg(target_os = "windows")]
	config
		.interpreter_config
		.module_search_paths
		.as_mut()
		.unwrap()
		.push(dunce::canonicalize(folder.join("lib"))?);

	config.interpreter_config.run_filename = None;
	config.interpreter_config.argv = Some(vec![]);

	// Set 1T python home env variable for subprocesses
	std::env::set_var(
		"_QUERENT_PY_HOME",
		config.interpreter_config.home.as_ref().map(|p| p.as_os_str()).unwrap(),
	);

	Ok(config)
}
