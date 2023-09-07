//! Configuration for the plugin.

use std::{error::Error, path::PathBuf};
use tauri::api::path;

#[derive(Debug, Clone)]
/// Configuration for the tauri settings plugin.
pub struct Config {
	/// The name of the file in which the settings are stored (as JSON). (Default: `settings.json`)
	pub file_name: String,
	/// Path to the settings file
	pub file_path: PathBuf,
	/// The directory in which the settings file will be stored. (Default: App config directory)
	pub dir_path: PathBuf,
	/// Whether to prettify the JSON output. (Default: `false`)
	pub prettify: bool,
}

/// Configuration options for the tauri settings plugin.
#[derive(Debug, serde::Deserialize, Clone)]
pub struct ConfigOptions {
	/// The name of the file in which the settings are stored (as JSON). (Default: `settings.json`)
	pub file_name: Option<String>,
	/// The directory in which the settings file will be stored. (Default: App config directory)
	pub dir_path: Option<PathBuf>,
	/// Whether to prettify the JSON output. (Default: `false`)
	pub prettify: Option<bool>,
}

impl ConfigOptions {
	/// Creates a new ConfigOptions struct.
	///
	/// ### Examples
	/// ```no_run
	/// # use tauri_plugin_settings::ConfigOptions;
	/// let config = ConfigOptions::new(
	///     Some("user-settings.json".into()), // File in which the settings are saved
	///     None, // Config directory
	///     Some(true), // Whether to prettify the JSON
	/// );
	/// ```
	pub fn new(
		file_name: Option<String>,
		dir_path: Option<PathBuf>,
		prettify: Option<bool>,
	) -> ConfigOptions {
		ConfigOptions {
			file_name,
			dir_path,
			prettify,
		}
	}
}

impl Config {
	/// Creates a new Config struct.
	///
	/// ### Arguments
	/// * `app_config`: The tauri app config.
	/// * `file_name`: The name of the JSON file (with the extension) where the settings will be stored. (Default: `settings.json`)
	/// * `directory`: Path to the directory where the settings file will be stored. (Default: [App config directory](https://tauri.app/v1/api/js/path#appconfigdir))
	/// * `prettify`: Whether to prettify the JSON settings data. (Default: `false`)
	///
	/// ### Examples
	/// ```no_run
	/// # use tauri_plugin_settings::Config;
	/// # let app_config = tauri::Config::default();
	/// // Where app_config is tauri::Config
	/// let config = Config::new(
	///     &app_config,
	///     Some("user-settings.json".into()), // File in which the settings are saved
	///     None, // Config directory
	///     Some(true), // Whether to prettify the JSON
	/// );
	/// ```
	pub fn new(
		app_config: &tauri::Config,
		file_name: Option<String>,
		dir_path: Option<PathBuf>,
		prettify: Option<bool>,
	) -> Result<Config, Box<dyn Error>> {
		let config_directory = dir_path.unwrap_or(
			path::app_config_dir(app_config).ok_or("Error: Default config directory not found.")?,
		);

		let file_name = file_name.unwrap_or("settings.json".to_string());
		let file_path = config_directory.join(&file_name);

		Ok(Config {
			file_name,
			file_path,
			dir_path: config_directory,
			prettify: prettify.unwrap_or(false),
		})
	}

	/// Creates a new Config from a ConfigOptions struct.
	///
	/// Defaults: See [`Config::new`].
	/// ### Examples
	/// ```no_run
	/// # use tauri_plugin_settings::{Config, ConfigOptions};
	/// # let app_config = tauri::Config::default();
	/// // Where app_config is tauri::Config
	///
	/// let config_options = ConfigOptions::new(Some("preferences.json".into()), None, Some(true));
	/// let config = Config::from_config_options(&app_config, &config_options);
	/// ```
	pub fn from_config_options(
		app_config: &tauri::Config,
		options: &ConfigOptions,
	) -> Result<Config, Box<dyn Error>> {
		Config::new(
			app_config,
			options.file_name.clone(),
			options.dir_path.clone(),
			options.prettify,
		)
	}

	/// Creates a new Config with the default values.
	///
	/// All `None` fields are replaced by their default values.
	///
	/// Default: See [`Config::new`].
	///
	/// ### Examples
	/// ```no_run
	/// # use tauri_plugin_settings::Config;
	/// # let app_config = tauri::Config::default();
	/// // Where app_config is tauri::Config
	/// let config = Config::default(&app_config);
	/// ```
	pub fn default(app_config: &tauri::Config) -> Result<Config, Box<dyn Error>> {
		Config::new(app_config, None, None, None)
	}
}
