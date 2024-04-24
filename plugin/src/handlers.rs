//! Tauri plugin handlers/commands.

use serde_json::Value;
use tauri::{AppHandle, Runtime, State};

use crate::{settings::{SettingsFile, SettingsFileOptions}, PluginState};

#[tauri::command]
pub(crate) fn add_settings_file<R: Runtime>(
	_app: AppHandle<R>,
	state: State<'_, PluginState>,
	settings_file_options: SettingsFileOptions
) -> Result<usize, String> {
	let mut state = state.inner().lock().map_err(|err| err.to_string())?;

	if !state.plugin_config.allow_file_addition {
		return Err("Error: Settings file addition from frontend is not allowed.".into());
	}

	if state.plugin_config.files_limit != 0
		&& state.plugin_config.files_limit >= state.settings_files.len()
	{
		return Err("Error: Settings file limit reached.".into());
	}

	let settings_file_path = state.plugin_config.scope.join(settings_file_options.scoped_file_path);

	if settings_file_path.starts_with(&state.plugin_config.scope) {
		return Err("Error: Settings file path out of the allowed scope.".into());
	}

	let settings_file = SettingsFile::new(
		settings_file_path,
		settings_file_options.prettify,
		settings_file_options.default_settings
	).map_err(|err| err.to_string())?;

	Ok(state.add_settings_file(settings_file))
}

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
	file_id: Option<usize>,
) -> Result<bool, String> {
	let state = state.inner().lock().map_err(|err| err.to_string())?;
	let settings_file = state
		.get_settings_file(file_id.unwrap_or(0))
		.map_err(|err| err.to_string())?;

	settings_file.has(key).map_err(|err| err.to_string())
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
	file_id: Option<usize>,
) -> Result<Value, String> {
	let state = state.inner().lock().map_err(|err| err.to_string())?;
	let settings_file = state
		.get_settings_file(file_id.unwrap_or(0))
		.map_err(|err| err.to_string())?;

	settings_file.get(key).map_err(|err| err.to_string())
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
	file_id: Option<usize>,
) -> Result<(), String> {
	let mut state = state.inner().lock().map_err(|err| err.to_string())?;
	let settings_file = state
		.get_settings_file_mut(file_id.unwrap_or(0))
		.map_err(|err| err.to_string())?;

	settings_file.set(key, value).map_err(|err| err.to_string())
}
