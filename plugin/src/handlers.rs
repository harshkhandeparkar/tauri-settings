//! Tauri plugin handlers/commands.

use serde_json::{to_value, Value};
use tauri::{AppHandle, Runtime, State};

use crate::{
	config::{Config, ConfigOptions},
	dot_notation::{get_dot_notation, set_dot_notation},
	fs::{load_settings_file, save_settings_json},
	settings, PluginState, PluginStateConfig,
};

#[tauri::command]
pub(crate) fn add_config<R: Runtime>(
	app: AppHandle<R>,
	state: State<'_, PluginState>,
	config: ConfigOptions,
	default_settings: Value,
) -> Result<(u32, Value), String> {
	let mut state = state.inner().lock().map_err(|err| err.to_string())?;

	let config =
		Config::from_config_options(&app.config(), &config).map_err(|err| err.to_string())?;

	let (loaded_settings, _, was_created) =
		load_settings_file(&config).map_err(|err| err.to_string())?;

	let settings: Value = if was_created {
		save_settings_json(&default_settings, &config).map_err(|err| err.to_string())?;
		default_settings
	} else {
		loaded_settings
	};

	let config_id = state.add_config(PluginStateConfig {
		config,
		settings_cache: settings.clone(),
	});

	Ok((config_id, settings))
}

#[tauri::command]
pub(crate) fn has<R: Runtime>(
	_app: AppHandle<R>,
	state: State<'_, PluginState>,
	key: &str,
	config_id: Option<u32>,
) -> Result<bool, String> {
	let mut state = state.inner().lock().map_err(|err| err.to_string())?;
	let state = state
		.get_config_mut(config_id.unwrap_or(0))
		.map_err(|err| err.to_string())?;

	let (exists, new_settings) =
		settings::_has(&state.config, key).map_err(|err| err.to_string())?;
	state.settings_cache = new_settings;

	Ok(exists)
}

#[tauri::command]
pub(crate) fn has_cache<R: Runtime>(
	_app: AppHandle<R>,
	state: State<'_, PluginState>,
	key: &str,
	config_id: Option<u32>,
) -> Result<bool, String> {
	let state = state.inner().lock().map_err(|err| err.to_string())?;
	let state = state
		.get_config(config_id.unwrap_or(0))
		.map_err(|err| err.to_string())?;

	let value: Value =
		get_dot_notation(&state.settings_cache, key).map_err(|err| err.to_string())?;

	Ok(!value.is_null())
}

#[tauri::command]
pub(crate) fn get<R: Runtime>(
	_app: AppHandle<R>,
	state: State<'_, PluginState>,
	key: &str,
	config_id: Option<u32>,
) -> Result<Value, String> {
	let mut state = state.inner().lock().map_err(|err| err.to_string())?;
	let state = state
		.get_config_mut(config_id.unwrap_or(0))
		.map_err(|err| err.to_string())?;

	let (value, new_settings) =
		settings::_get(&state.config, key).map_err(|err| err.to_string())?;
	state.settings_cache = new_settings;

	Ok(value)
}

#[tauri::command]
pub(crate) fn get_cache<R: Runtime>(
	_app: AppHandle<R>,
	state: State<'_, PluginState>,
	key: &str,
	config_id: Option<u32>,
) -> Result<Value, String> {
	let state = state.inner().lock().map_err(|err| err.to_string())?;
	let state = state
		.get_config(config_id.unwrap_or(0))
		.map_err(|err| err.to_string())?;

	let value: Value =
		get_dot_notation(&state.settings_cache, key).map_err(|err| err.to_string())?;

	Ok(value)
}

#[tauri::command]
pub(crate) fn set<R: Runtime>(
	_app: AppHandle<R>,
	state: State<'_, PluginState>,
	key: &str,
	value: Value,
	config_id: Option<u32>,
) -> Result<Value, String> {
	let mut state = state.inner().lock().map_err(|err| err.to_string())?;
	let state = state
		.get_config_mut(config_id.unwrap_or(0))
		.map_err(|err| err.to_string())?;

	let new_settings = settings::set(&state.config, key, value).map_err(|err| err.to_string())?;
	state.settings_cache = new_settings.clone();

	Ok(new_settings)
}

#[tauri::command]
pub(crate) fn set_cache<R: Runtime>(
	_app: AppHandle<R>,
	state: State<'_, PluginState>,
	key: &str,
	new_value: Value,
	config_id: Option<u32>,
) -> Result<Value, String> {
	let state = state.inner().lock().map_err(|err| err.to_string())?;
	let state = state
		.get_config(config_id.unwrap_or(0))
		.map_err(|err| err.to_string())?;

	let new_settings = set_dot_notation(
		&state.settings_cache,
		key,
		to_value(new_value).map_err(|err| err.to_string())?,
	)
	.map_err(|err| err.to_string())?;

	Ok(new_settings)
}

#[tauri::command]
pub(crate) fn cache_to_file<R: Runtime>(
	_app: AppHandle<R>,
	state: State<'_, PluginState>,
	config_id: Option<u32>,
) -> Result<(), String> {
	let state = state.inner().lock().map_err(|err| err.to_string())?;
	let state = state
		.get_config(config_id.unwrap_or(0))
		.map_err(|err| err.to_string())?;

	save_settings_json(&state.settings_cache, &state.config).map_err(|err| err.to_string())?;

	Ok(())
}

#[tauri::command]
pub(crate) fn file_to_cache<R: Runtime>(
	_app: AppHandle<R>,
	state: State<'_, PluginState>,
	config_id: Option<u32>,
) -> Result<(), String> {
	let mut state = state.inner().lock().map_err(|err| err.to_string())?;
	let state = state
		.get_config_mut(config_id.unwrap_or(0))
		.map_err(|err| err.to_string())?;

	let (settings, _, _) = load_settings_file(&state.config).map_err(|err| err.to_string())?;

	state.settings_cache = settings;

	Ok(())
}
