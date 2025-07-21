//! Configuration for the plugin.

use std::{error::Error, path::PathBuf};
use tauri::{AppHandle, Manager, Runtime};

#[derive(Debug, Clone)]
/// A struct that stores the configuration for the tauri settings plugin.
pub struct PluginConfig {
	/// A path to limit the creation of settings files to.
	pub scope: PathBuf,
	/// Whether to allow the frontend handlers to create new settings files.
	pub allow_file_addition: bool,
	/// The maximum number of settings files that can be created by the frontend handlers. Setting it to `0` removes the limit.
	pub files_limit: usize,
}

#[derive(Debug, Clone)]
/// Configuration options for the tauri settings plugin.
pub struct PluginConfigOptions {
	/// A path to limit the creation of settings files to. Default: The app config directory.
	pub scope: Option<PathBuf>,
	/// Whether to allow the frontend handlers to create new settings files. Default: `false`.`
	pub allow_file_addition: Option<bool>,
	/// The maximum number of settings files that can be created by the frontend handlers. Setting it to `0` removes the limit. Default: `0`.
	pub files_limit: Option<usize>,
}

impl PluginConfigOptions {
	/// Creates a new `PluginConfigOptions` struct.
	pub fn new(
		scope: Option<PathBuf>,
		files_limit: Option<usize>,
		allow_file_addition: Option<bool>,
	) -> Self {
		Self {
			scope,
			files_limit,
			allow_file_addition,
		}
	}
}

impl PluginConfig {
	/// Creates a new `PluginConfig` struct.
	/// ### Arguments
	/// - `app_config`: The Tauri app config, used to get the path of the system app config directory.
	/// - `scope`: A directory to which all settings files will be restricted. Default: The system app config directory.
	/// - `files_limit`: The maximum number of settings files that can be created (from the frontend). Use `0` for unlimited. Default: `0`.
	/// - `allow_file_addition`: Whether to allow the addition of settings files from the frontend. Default: `false`.
	pub fn new<R: Runtime>(
		app: &AppHandle<R>,
		scope: Option<PathBuf>,
		files_limit: Option<usize>,
		allow_file_addition: Option<bool>,
	) -> Result<Self, Box<dyn Error>> {
		let scope = scope.unwrap_or(
			app.path().app_config_dir().map_err(|_| "Error: Default config directory not found.")?,
		);

		Ok(Self {
			scope,
			files_limit: files_limit.unwrap_or(0),
			allow_file_addition: allow_file_addition.unwrap_or(false),
		})
	}

	/// Creates a new `PluginConfig` struct from `PluginConfigOptions` struct, replacing all `None` values with their defaults.
	pub fn from_options<R: Runtime>(
		app: &AppHandle<R>,
		options: &PluginConfigOptions,
	) -> Result<Self, Box<dyn Error>> {
		Self::new(
			app,
			options.scope.clone(),
			options.files_limit,
			options.allow_file_addition,
		)
	}

	/// Creates a `PluginConfig` struct with the default values.
	pub fn default<R: Runtime>(app: &AppHandle<R>) -> Result<Self, Box<dyn Error>> {
		Self::new(app, None, None, None)
	}
}
