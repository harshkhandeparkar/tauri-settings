use std::error::Error;
use tauri::api::path;

#[derive(Debug, Default, Clone)]
pub struct Config {
	pub file_name: String,
	pub directory: String,
	pub prettify: bool,
	pub num_spaces: u8,
}

#[derive(Debug, Default, serde::Deserialize, Clone)]
pub struct ConfigOptions {
	pub file_name: Option<String>,
	pub directory: Option<String>,
	pub prettify: Option<bool>,
	pub num_spaces: Option<u8>,
}

impl Config {
	pub fn new(
		app_config: &tauri::Config,
		file_name: Option<String>,
		directory: Option<String>,
		prettify: Option<bool>,
		num_spaces: Option<u8>,
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
			num_spaces: num_spaces.unwrap_or(0),
		})
	}

	pub fn from_config_options(
		app_config: &tauri::Config,
		options: &ConfigOptions,
	) -> Result<Config, Box<dyn Error>> {
		Config::new(
			app_config,
			options.file_name.clone(),
			options.directory.clone(),
			options.prettify,
			options.num_spaces,
		)
	}
}
