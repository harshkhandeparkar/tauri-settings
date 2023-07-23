use std::error::Error;

use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_value, to_value, Value};

use crate::{
	config::Config,
	dot_notation::{get_dot_notation, set_dot_notation},
	fs::{load_settings_json, save_settings_json},
};

pub trait SettingsSchema: Sized + Serialize + DeserializeOwned + Default + Copy {}

pub fn has(config: &Config, key: &str) -> Result<bool, String> {
	let (settings_json, _) = load_settings_json(config).map_err(|err| err.to_string())?;

	let settings: Value = serde_json::from_str(&settings_json).map_err(|err| err.to_string())?;

	let value: Value = get_dot_notation(&settings, key.into()).map_err(|err| err.to_string())?;

	Ok(!value.is_null())
}

pub fn get(config: &Config, key: &str) -> Result<Value, String> {
	let (settings_json, _) = load_settings_json(config).map_err(|err| err.to_string())?;

	let settings: Value = serde_json::from_str(&settings_json).map_err(|err| err.to_string())?;

	get_dot_notation(&settings, key.into()).map_err(|err| err.to_string())
}

pub fn set(config: &Config, key: &str, value: Value) -> Result<Value, String> {
	let (settings_json, _) = load_settings_json(config).map_err(|err| err.to_string())?;

	let settings: Value = serde_json::from_str(&settings_json).map_err(|err| err.to_string())?;

	let new_settings =
		set_dot_notation(&settings, key.into(), value).map_err(|err| err.to_string())?;
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
		num_spaces: Option<u8>,
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

	pub fn has_cache<T: SettingsSchema>(&self, key: &str) -> Result<bool, Box<dyn Error>> {
		let value: T = self.get_cache(key)?;
		let value = to_value(value)?;

		Ok(!value.is_null())
	}

	pub fn get_cache<T: SettingsSchema>(&self, key: &str) -> Result<T, Box<dyn Error>> {
		get_dot_notation(&self.settings, key.into())
	}

	pub fn set_cache(&mut self, key: &str, new_value: Value) -> Result<S, Box<dyn Error>> {
		let new_settings: S = set_dot_notation(&self.settings, key.into(), new_value)?;

		self.settings = new_settings;
		Ok(new_settings)
	}
}
