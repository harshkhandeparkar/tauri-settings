//! Configuration for the plugin.

use std::{error::Error, path::PathBuf};
use tauri::api::path;

#[derive(Debug, Clone)]
/// Configuration for the tauri settings plugin.
pub struct PluginConfig {
	pub scope: PathBuf,
	pub files_limit: u32,
	pub allow_file_addition: bool,
}

#[derive(Debug, Clone)]
pub struct PluginConfigOptions {
	pub scope: Option<PathBuf>,
	pub files_limit: Option<u32>,
	pub allow_file_addition: Option<bool>,
}

impl PluginConfig {
	pub fn new(
		app_config: &tauri::Config,
		scope: Option<PathBuf>,
		files_limit: Option<u32>,
		allow_file_addition: Option<bool>,

	) -> Result<Self, Box<dyn Error>> {
		let scope = scope.unwrap_or(
			path::app_config_dir(app_config).ok_or("Error: Default config directory not found.")?,
		);

		Ok(Self {
			scope,
			files_limit: files_limit.unwrap_or(0),
			allow_file_addition: allow_file_addition.unwrap_or(false)
		})
	}

	pub fn from_options(
		app_config: &tauri::Config,
		options: &PluginConfigOptions,
	) -> Result<Self, Box<dyn Error>> {
		Self::new(
			app_config,
			options.scope.clone(),
			options.files_limit,
			options.allow_file_addition
		)
	}
}