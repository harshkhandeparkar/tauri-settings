use crate::config::Config;
use std::{error::Error, fs, path::Path};

pub fn ensure_settings_file(config: &Config) -> Result<bool, std::io::Error> {
	let settings_dir_path = Path::new(&config.directory);
	let settings_file_path = settings_dir_path.join(&config.file_name);

	if !settings_file_path.exists() {
		if !settings_dir_path.exists() {
			fs::create_dir_all(settings_dir_path)?;
		}

		fs::write(settings_file_path, "{}")?;
		return Ok(true);
	}

	Ok(false)
}

pub fn load_settings_json(config: &Config) -> Result<(String, String, bool), Box<dyn Error>> {
	let was_created = ensure_settings_file(config)?;

	let settings_file_path = Path::new(&config.directory).join(&config.file_name);
	let settings_json = fs::read_to_string(&settings_file_path)?;

	let settings_file_path: String = settings_file_path
		.to_str()
		.unwrap_or(&config.file_name)
		.to_string();

	Ok((settings_json, settings_file_path, was_created))
}

pub fn save_settings_json<T: ?Sized + serde::Serialize>(
	settings: &T,
	config: &Config,
) -> Result<(), Box<dyn Error>> {
	let settings_file_path = Path::new(&config.directory).join(&config.file_name);

	let settings_json = if config.prettify {
		serde_json::to_string_pretty(&settings)?
	} else {
		serde_json::to_string(&settings)?
	};

	fs::write(settings_file_path, settings_json)?;
	Ok(())
}
