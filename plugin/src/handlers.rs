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
) -> Result<bool, String> {
	let config = state.inner();
	settings::has(config, key).map_err(|err| err.to_string())
}

#[tauri::command]
pub fn get<R: Runtime>(
	_app: AppHandle<R>,
	state: State<'_, Config>,
	key: &str,
) -> Result<Value, String> {
	let config = state.inner();
	settings::get(config, key).map_err(|err| err.to_string())
}

#[tauri::command]
pub fn set<R: Runtime>(
	_app: AppHandle<R>,
	state: State<'_, Config>,
	key: &str,
	value: Value,
) -> Result<Value, String> {
	let config = state.inner();
	settings::set(config, key, value).map_err(|err| err.to_string())
}

#[tauri::command]
pub fn overwrite_settings<R: Runtime>(
	_app: AppHandle<R>,
	state: State<'_, Config>,
	new_settings: Value,
) -> Result<(), String> {
	let config = state.inner();
	save_settings_json(&new_settings, &config).map_err(|err| err.to_string())?;

	Ok(())
}

#[tauri::command]
pub fn read_settings<R: Runtime>(
	_app: AppHandle<R>,
	state: State<'_, Config>,
) -> Result<(Value, bool), String> {
	let config = state.inner();
	let (settings_json, was_created) = load_settings_json(config).map_err(|err| err.to_string())?;

	let settings: Value = serde_json::from_str(&settings_json).map_err(|err| err.to_string())?;

	Ok((settings, was_created))
}
