//! Tauri plugin handlers/commands.

use serde_json::Value;
use tauri::{AppHandle, Runtime, State};

use crate::PluginState;

/// Checks whether a key exists in the settings.
///
/// ### Arguments
/// * `key`: Key for the setting. Supports dot notation. (e.g. `preferences.theme`)
/// * `config_id`: ID for an optional custom configuration. Selects the default/initial configuration if absent.
#[tauri::command]
pub(crate) fn has<R: Runtime>(
	_app: AppHandle<R>,
	state: State<'_, PluginState>,
	key: &str,
	file_id: Option<u32>,
) -> Result<bool, String> {
	let state = state.inner().lock().map_err(|err| err.to_string())?;
	let settings_file = state
		.get_settings_file(file_id.unwrap_or(0))
		.map_err(|err| err.to_string())?;

	Ok(settings_file.has(key).map_err(|err| err.to_string())?)
}

/// Gets the value corresponding to a key in the settings.
///
/// ### Arguments
/// * `key`: Key for the setting. Supports dot notation. (e.g. `preferences.theme`)
/// * `config_id`: ID for an optional custom configuration. Selects the default/initial configuration if absent.
#[tauri::command]
pub(crate) fn get<R: Runtime>(
	_app: AppHandle<R>,
	state: State<'_, PluginState>,
	key: &str,
	file_id: Option<u32>,
) -> Result<Value, String> {
	let state = state.inner().lock().map_err(|err| err.to_string())?;
	let settings_file = state
		.get_settings_file(file_id.unwrap_or(0))
		.map_err(|err| err.to_string())?;

	Ok(settings_file.get(key).map_err(|err| err.to_string())?)
}

/// Sets the value corresponding to a key in the settings. Creates the key path recursively if it doesn't exist.
///
/// ### Arguments
/// * `key`: Key for the setting. Supports dot notation. (e.g. `preferences.theme`)
/// * `value`: The value to set.
/// * `config_id`: ID for an optional custom configuration. Selects the default/initial configuration if absent.
#[tauri::command]
pub(crate) fn set<R: Runtime>(
	_app: AppHandle<R>,
	state: State<'_, PluginState>,
	key: &str,
	value: Value,
	file_id: Option<u32>,
) -> Result<(), String> {
	let mut state = state.inner().lock().map_err(|err| err.to_string())?;
	let settings_file = state
		.get_settings_file_mut(file_id.unwrap_or(0))
		.map_err(|err| err.to_string())?;

	settings_file.set(key, value).map_err(|err| err.to_string())
}
