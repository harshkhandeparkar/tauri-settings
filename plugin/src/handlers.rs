use serde_json::Value;
use tauri::{AppHandle, Runtime, State};

use crate::{
	config::Config,
	fs::{load_settings_json, save_settings_json},
	settings,
};

#[tauri::command]
pub fn has<R: Runtime>(
	_app: AppHandle<R>,
	state: State<'_, Config>,
	key: &str,
	custom_config: Option<Config>,
) -> Result<bool, String> {
	let config = &custom_config.unwrap_or_else(|| state.inner().clone());
	settings::has(config, key).map_err(|err| err.to_string())
}

#[tauri::command]
pub fn get<R: Runtime>(
	_app: AppHandle<R>,
	state: State<'_, Config>,
	key: &str,
	custom_config: Option<Config>,
) -> Result<Value, String> {
	let config = &custom_config.unwrap_or_else(|| state.inner().clone());
	settings::get(config, key).map_err(|err| err.to_string())
}

#[tauri::command]
pub fn set<R: Runtime>(
	_app: AppHandle<R>,
	state: State<'_, Config>,
	key: &str,
	value: Value,
	custom_config: Option<Config>,
) -> Result<Value, String> {
	let config = &custom_config.unwrap_or_else(|| state.inner().clone());
	settings::set(config, key, value).map_err(|err| err.to_string())
}

#[tauri::command]
pub fn overwrite_settings<R: Runtime>(
	_app: AppHandle<R>,
	state: State<'_, Config>,
	new_settings: Value,
	custom_config: Option<Config>,
) -> Result<(), String> {
	let config = &custom_config.unwrap_or_else(|| state.inner().clone());
	save_settings_json(&new_settings, &config).map_err(|err| err.to_string())?;

	Ok(())
}

#[tauri::command]
pub fn read_settings<R: Runtime>(
	_app: AppHandle<R>,
	state: State<'_, Config>,
	custom_config: Option<Config>,
) -> Result<(Value, String, bool), String> {
	let config = &custom_config.unwrap_or_else(|| state.inner().clone());
	let (settings_json, settings_file_path, was_created) =
		load_settings_json(config).map_err(|err| err.to_string())?;

	let settings: Value = serde_json::from_str(&settings_json).map_err(|err| err.to_string())?;

	Ok((settings, settings_file_path, was_created))
}
