use std::error::Error;

use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_value, to_value, Value};

use crate::{
	config::Config,
	dot_notation::{get_dot_notation, set_dot_notation},
	fs::{load_settings_json, save_settings_json},
};

pub trait SettingsSchema: Serialize + DeserializeOwned + Default + Clone {}

/// Checks if a key exists in the settings.
///
/// Here key supports dot notation. Eg: `preferences.theme`.
/// ### Examples
/// ```no_run
/// # use tauri_plugin_settings::{Config, settings::has};
/// # let config = Config::default(&tauri::Config::default()).unwrap();
/// let theme_exists = has(&config, "preferences.theme").unwrap();
/// ```
pub fn has(config: &Config, key: &str) -> Result<bool, Box<dyn Error>> {
	let (exists, _) = _has(config, key)?;

	Ok(exists)
}

pub(crate) fn _has(config: &Config, key: &str) -> Result<(bool, Value), Box<dyn Error>> {
	let (settings_json, _, _) = load_settings_json(config)?;

	let settings: Value = serde_json::from_str(&settings_json)?;

	let value: Value = get_dot_notation(&settings, key)?;

	Ok((!value.is_null(), settings))
}

/// Returns the value corresponding to a key in the settings.
///
/// Here key supports dot notation. Eg: `preferences.theme`.
/// ### Examples
/// ```no_run
/// # use tauri_plugin_settings::{Config, settings::get};
/// # let config = Config::default(&tauri::Config::default()).unwrap();
/// let theme: String = get(&config, "preferences.theme").unwrap();
/// ```
///
/// ```no_run
/// # use tauri_plugin_settings::{Config, settings::get};
/// # let config = Config::default(&tauri::Config::default()).unwrap();
/// let theme: Vec<String> = get(&config, "recently_opened").unwrap();
/// ```
pub fn get<V: DeserializeOwned>(config: &Config, key: &str) -> Result<V, Box<dyn Error>> {
	let (value, _) = _get(config, key)?;

	Ok(value)
}

pub fn _get<V: DeserializeOwned>(config: &Config, key: &str) -> Result<(V, Value), Box<dyn Error>> {
	let (settings_json, _, _) = load_settings_json(config)?;

	let settings: Value = serde_json::from_str(&settings_json)?;

	Ok((from_value(get_dot_notation(&settings, key)?)?, settings))
}

/// Sets the value corresponding to a key in the settings.
///
/// Here key supports dot notation. Eg: `preferences.theme`.
/// ### Examples
/// ```no_run
/// # use tauri_plugin_settings::{Config, settings::set};
/// # let config = Config::default(&tauri::Config::default()).unwrap();
/// set(&config, "preferences.theme", "dark").unwrap();
/// ```
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