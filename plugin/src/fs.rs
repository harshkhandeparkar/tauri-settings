//! Helper functions for loading and saving settings files.

use serde_json::Value;

use crate::config::Config;
use std::{error::Error, fs};

/// Ensures that the settings file exists, and creates it if it doesn't.
///
/// Returns whether a new file was created. The default contents for a new file are `{}`.
///
/// ### Examples
/// ```ignore
/// let file_was_created: bool = ensure_settings_file(&config).unwrap();
/// ```
pub fn ensure_settings_file(config: &Config) -> Result<bool, std::io::Error> {
	if !config.file_path.exists() {
		if !config.dir_path.exists() {
			fs::create_dir_all(&config.dir_path)?;
		}

		fs::write(&config.file_path, "{}")?;
		return Ok(true);
	}

	Ok(false)
}

/// Loads the settings from the settings JSON file. Creates the file if it doesn't exist.
///
/// Returns a tuple with the following fields:
/// * The settings as a [`serde_json::Value`].
/// * The path to the settings file as a [`String`].
/// * Whether a new file with default values was created.
///
/// ### Examples
/// ```ignore
/// let (settings, settings_file_path, was_created) = load_settings_file(&config).unwrap();
/// ```
pub fn load_settings_file(config: &Config) -> Result<Value, Box<dyn Error>> {
	let settings_json = fs::read_to_string(&config.file_path)?;

	let settings: serde_json::Value = serde_json::from_str(&settings_json)?;

	Ok(settings)
}

/// Saves the settings to the settings JSON file. Creates the file if it doesn't exist.
///
/// ### Examples
/// ```ignore
/// save_settings_json(Settings { theme: "dark" }, &config).unwrap();
/// ```
pub fn save_settings_json<T: ?Sized + serde::Serialize>(
	settings: &T,
	config: &Config,
) -> Result<(), Box<dyn Error>> {
	let settings_json = if config.prettify {
		serde_json::to_string_pretty(&settings)?
	} else {
		serde_json::to_string(&settings)?
	};

	fs::write(&config.file_path, settings_json)?;
	Ok(())
}
