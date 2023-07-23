use std::error::Error;

use serde_json::Value;

use crate::{
	config::Config,
	dot_notation::{get_dot_notation, set_dot_notation},
	fs::{load_settings_json, save_settings_json},
};

pub fn has(config: &Config, key: &str) -> Result<bool, String> {
	let (settings_json, _) = load_settings_json(config).map_err(|err| err.to_string())?;

	let settings: Value = serde_json::from_str(&settings_json).map_err(|err| err.to_string())?;

	let value = get_dot_notation(&settings, key.into());

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
