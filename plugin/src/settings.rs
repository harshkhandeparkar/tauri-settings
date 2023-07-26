use std::error::Error;

use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_value, to_value, Value};

use crate::{
	config::Config,
	dot_notation::{get_dot_notation, set_dot_notation},
	fs::{load_settings_json, save_settings_json},
};

pub trait SettingsSchema: Sized + Serialize + DeserializeOwned + Default + Copy {}

/// Checks if a key exists in the settings.
///
/// Here key supports dot notation. Eg: `preferences.theme`.
/// ### Examples
/// ```no_run
/// # use tauri_plugin_settings::{Config, settings::has};
/// # let config = Config::new(&tauri::Config::default(), None, None, None).unwrap();
/// let theme_exists = has(&config, "preferences.theme").unwrap();
/// ```
pub fn has(config: &Config, key: &str) -> Result<bool, Box<dyn Error>> {
	let (settings_json, _, _) = load_settings_json(config)?;

	let settings: Value = serde_json::from_str(&settings_json)?;

	let value: Value = get_dot_notation(&settings, key)?;

	Ok(!value.is_null())
}

pub fn get<V: DeserializeOwned>(config: &Config, key: &str) -> Result<V, Box<dyn Error>> {
	let (settings_json, _, _) = load_settings_json(config)?;

	let settings: Value = serde_json::from_str(&settings_json)?;

	Ok(from_value(get_dot_notation(&settings, key)?)?)
}

pub fn set<V: Serialize>(
	config: &Config,
	key: &str,
	new_value: V,
) -> Result<Value, Box<dyn Error>> {
	let (settings_json, _, _) = load_settings_json(config)?;

	let settings: Value = serde_json::from_str(&settings_json)?;

	let new_settings = set_dot_notation(&settings, key, to_value(new_value)?)?;
	save_settings_json(&new_settings, config)?;

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
	) -> Result<SettingsManager<S>, Box<dyn Error>> {
		let config = Config::new(app_config, file_name, directory, prettify)?;

		Ok(SettingsManager {
			settings: S::default(),
			config,
		})
	}

	pub fn initialize(&mut self) -> Result<(), Box<dyn Error>> {
		let (current_settings_json, _, was_created) = load_settings_json(&self.config)?;

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
		get_dot_notation(&self.settings, key)
	}

	pub fn set_cache<V: Serialize>(
		&mut self,
		key: &str,
		new_value: V,
	) -> Result<S, Box<dyn Error>> {
		let new_settings: S = set_dot_notation(&self.settings, key, new_value)?;

		self.settings = new_settings;
		Ok(new_settings)
	}

	pub fn get<V: Serialize + DeserializeOwned + Clone>(
		&mut self,
		key: &str,
	) -> Result<V, Box<dyn Error>> {
		let value: V = get(&self.config, key)?;

		self.set_cache(key, value.clone())?;

		Ok(value)
	}

	pub fn set<V: Serialize>(&mut self, key: &str, new_value: V) -> Result<S, Box<dyn Error>> {
		let new_settings: S = from_value(set(&self.config, key, new_value)?)?;
		self.settings = new_settings;

		Ok(new_settings)
	}

	pub fn sync_cache(&self) -> Result<S, Box<dyn Error>> {
		save_settings_json(&self.settings, &self.config)?;

		Ok(self.settings)
	}
}
