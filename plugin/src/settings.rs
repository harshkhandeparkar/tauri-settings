use std::error::Error;

use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_value, Value};

use crate::{
	config::Config,
	dot_notation::{get_dot_notation, set_dot_notation},
	fs::{load_settings_json, save_settings_json},
};

pub trait SettingsSchema: Sized + Serialize + DeserializeOwned + Default {}

pub fn has(config: &Config, key: &str) -> Result<bool, String> {
	let (settings_json, _) = load_settings_json(config).map_err(|err| err.to_string())?;

	let settings: Value = serde_json::from_str(&settings_json).map_err(|err| err.to_string())?;

	let value: Value = get_dot_notation(&settings, key.into());

	Ok(value.is_null())
}

pub fn get(config: &Config, key: &str) -> Result<Value, String> {
	let (settings_json, _) = load_settings_json(config).map_err(|err| err.to_string())?;

	let settings: Value = serde_json::from_str(&settings_json).map_err(|err| err.to_string())?;

	Ok(get_dot_notation(&settings, key.into()))
}

pub fn set(config: &Config, key: &str, value: Value) -> Result<Value, String> {
	let (settings_json, _) = load_settings_json(config).map_err(|err| err.to_string())?;

	let settings: Value = serde_json::from_str(&settings_json).map_err(|err| err.to_string())?;

	let new_settings = set_dot_notation(settings, key.into(), value);
	save_settings_json(&new_settings, config).map_err(|err| err.to_string())?;

	Ok(new_settings)
}

pub struct SettingsManager<S: SettingsSchema> {
	settings: S,
	config: Config,
}

impl<S: SettingsSchema> SettingsManager<S> {
	pub fn new(
		app_config: &tauri::Config,
		file_name: Option<String>,
		directory: Option<String>,
		prettify: Option<bool>,
		num_spaces: Option<u8>
	) -> Result<SettingsManager<S>, Box<dyn Error>> {
		let config = Config::new(app_config, file_name, directory, prettify, num_spaces)?;

		Ok(SettingsManager {
			settings: S::default(),
			config,
		})
	}

	pub fn initialize(&mut self) -> Result<(), Box<dyn Error>> {
		let (current_settings_json, was_created) = load_settings_json(&self.config)?;

		if was_created {
			save_settings_json(&self.settings, &self.config)?;
		} else {
			let new_settings: Value = serde_json::from_str(&current_settings_json)?;

			self.settings = from_value(new_settings)?;
		}

		Ok(())
	}
}
