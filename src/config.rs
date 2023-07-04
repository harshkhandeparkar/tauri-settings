use tauri;
use tauri::api::path;

pub struct Config {
	pub file_name: String,
	pub directory: String,
	pub prettify: bool,
	pub num_spaces: u8,
}

pub enum ConfigError {
	ConfigDirNotFound
}

impl Config {
	pub fn new(
		&self,
		app_config: &tauri::Config,
		file_name: Option<String>,
		directory: Option<String>,
		prettify: Option<bool>,
		num_spaces: Option<u8>,
	) -> Result<Config, ConfigError> {
		let mut config_directory: String = String::new();

		if let Some(dir) = directory {
			config_directory = dir;
		} else {
			if let Some(default_dir) = path::app_config_dir(app_config) {
				let default_dir_str = default_dir.to_str();

				if let Some(string) = default_dir_str {
					config_directory = string.to_string()
				} else {
					return Err(ConfigError::ConfigDirNotFound);
				}
			}
		}

		Ok(Config {
			file_name: file_name.unwrap_or("settings.json".to_string()),
			directory: config_directory,
			prettify: prettify.unwrap_or(false),
			num_spaces: num_spaces.unwrap_or(0),
		})
	}
}