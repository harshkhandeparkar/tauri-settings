use crate::config::{Config};
use std::{path::Path, fs, error::Error};

pub fn ensure_settings_file(config: Config) -> Result<(), std::io::Error> {
	let settings_dir_path = Path::new(&config.directory);
	let settings_file_path = settings_dir_path.join(&config.file_name);

	if !settings_file_path.exists() {
		if !settings_dir_path.exists() {
			fs::create_dir_all(settings_dir_path)?;
		}

		return fs::write(settings_file_path, "{}");
	}

	Ok(())
}

pub fn get_settings_json<T>(config: Config) -> Result<String, Box<dyn Error>> {
	let settings_file_path = Path::new(&config.directory).join(&config.file_name);

	let settings_json = fs::read_to_string(settings_file_path)?;

	Ok(settings_json)
}