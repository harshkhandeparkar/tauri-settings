use std::error::Error;
use tauri::api::path;

#[derive(Debug, Default, Clone)]

/// Configuration for the tauri settings plugin.
pub struct Config {
	/// The name of the file in which the settings are stored (as JSON). (Default: `settings.json`)
	pub file_name: String,
	/// The directory in which the settings file will be stored. (Default: App config directory)
	pub directory: String,
	/// Whether to prettify the JSON output. (Default: `false`)
	pub prettify: bool,
}

#[derive(Debug, Default, serde::Deserialize, Clone)]
pub struct ConfigOptions {
	/// The name of the file in which the settings are stored (as JSON). (Default: `settings.json`)
	pub file_name: Option<String>,
	/// The directory in which the settings file will be stored. (Default: App config directory)
	pub directory: Option<String>,
	/// Whether to prettify the JSON output. (Default: `false`)
	pub prettify: Option<bool>,
}

impl Config {
	/// Creates a new Config struct.
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
		directory: Option<String>,
		prettify: Option<bool>,
	) -> Result<Config, Box<dyn Error>> {
		let config_directory: String = directory.unwrap_or(
			path::app_config_dir(app_config)
				.ok_or("Error: Default config directory not found.")?
				.to_str()
				.ok_or("Error parsing default config directory.")?
				.to_string(),
		);

		Ok(Config {
			file_name: file_name.unwrap_or("settings.json".to_string()),
			directory: config_directory,
			prettify: prettify.unwrap_or(false),
		})
	}

	pub(crate) fn from_config_options(
		app_config: &tauri::Config,
		options: &ConfigOptions,
	) -> Result<Config, Box<dyn Error>> {
		Config::new(
			app_config,
			options.file_name.clone(),
			options.directory.clone(),
			options.prettify,
		)
	}
}
