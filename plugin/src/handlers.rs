use serde_json::Value;
use tauri::{AppHandle, Runtime, State};

use crate::{config::Config, fs::{load_settings_json, save_settings_json}, dot_notation::{get_dot_notation, set_dot_notation}};

#[tauri::command]
pub fn has<R: Runtime>(
	_app: AppHandle<R>,
	state: State<'_, Config>,
	key: &str
) -> Result<bool, String> {
	let config = state.inner();
	let settings_json = load_settings_json(config).map_err(|err| err.to_string())?;

	let settings: Value = serde_json::from_str(&settings_json).map_err(|err| err.to_string())?;

	let value = get_dot_notation(&settings, key.into());

	Ok(value.is_null())
}

#[tauri::command]
pub fn get<R: Runtime>(
	_app: AppHandle<R>,
	state: State<'_, Config>,
	key: &str
) -> Result<Value, String> {
	let config = state.inner();
	let settings_json = load_settings_json(config).map_err(|err| err.to_string())?;

	let settings: Value = serde_json::from_str(&settings_json).map_err(|err| err.to_string())?;

	Ok(get_dot_notation(&settings, key.into()))
}

#[tauri::command]
pub fn set<R: Runtime>(
	_app: AppHandle<R>,
	state: State<'_, Config>,
	key: &str,
	value: Value
) -> Result<Value, String> {
	let config = state.inner();
	let settings_json = load_settings_json(config).map_err(|err| err.to_string())?;

	let settings: Value = serde_json::from_str(&settings_json).map_err(|err| err.to_string())?;

	let new_settings = set_dot_notation(settings, key.into(), value);
	save_settings_json(new_settings.clone(), config).map_err(|err| err.to_string())?;

	Ok(new_settings)
}