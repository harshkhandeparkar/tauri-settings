use serde_json::{to_value, Value};
use tauri::{AppHandle, Runtime, State};

use crate::{
	config::{Config, ConfigOptions},
	dot_notation::{get_dot_notation, set_dot_notation},
	fs::{load_settings_json, save_settings_json},
	settings, PluginState, PluginStateConfig,
};

#[tauri::command]
pub fn has<R: Runtime>(
	app: AppHandle<R>,
	state: State<'_, PluginState>,
	key: &str,
	custom_config: Option<ConfigOptions>,
) -> Result<bool, String> {
	let config = &custom_config
		.as_ref()
		.map(|options| Config::from_config_options(&app.config(), &options))
		.unwrap_or_else(|| Ok(state.inner().lock()?.config.clone()))
		.map_err(|err| err.to_string())?;

	let (exists, new_settings) = settings::has(config, key).map_err(|err| err.to_string())?;
	if let None = custom_config {
		state
			.inner()
			.lock()
			.map_err(|err| err.to_string())?
			.settings = new_settings.clone();
	}

	Ok(exists)
}

#[tauri::command]
pub fn has_cache<R: Runtime>(
	_app: AppHandle<R>,
	state: State<'_, PluginState>,
	key: &str,
) -> Result<bool, String> {
	let settings_cache = &state
		.inner()
		.lock()
		.map_err(|err| err.to_string())?
		.settings;

	let value: Value = get_dot_notation(settings_cache, key).map_err(|err| err.to_string())?;

	Ok(!value.is_null())
}

#[tauri::command]
pub fn get<R: Runtime>(
	app: AppHandle<R>,
	state: State<'_, PluginState>,
	key: &str,
	custom_config: Option<ConfigOptions>,
) -> Result<Value, String> {
	let config = &custom_config
		.as_ref()
		.map(|options| Config::from_config_options(&app.config(), &options))
		.unwrap_or_else(|| Ok(state.inner().lock()?.config.clone()))
		.map_err(|err| err.to_string())?;

	let (value, new_settings) = settings::get(config, key).map_err(|err| err.to_string())?;
	if let None = custom_config {
		state
			.inner()
			.lock()
			.map_err(|err| err.to_string())?
			.settings = new_settings.clone();
	}

	Ok(value)
}

#[tauri::command]
pub fn get_cache<R: Runtime>(
	_app: AppHandle<R>,
	state: State<'_, PluginState>,
	key: &str,
) -> Result<Value, String> {
	let settings_cache = &state
		.inner()
		.lock()
		.map_err(|err| err.to_string())?
		.settings;

	let value: Value = get_dot_notation(settings_cache, key).map_err(|err| err.to_string())?;

	Ok(value)
}

#[tauri::command]
pub fn set<R: Runtime>(
	app: AppHandle<R>,
	state: State<'_, PluginState>,
	key: &str,
	value: Value,
	custom_config: Option<ConfigOptions>,
) -> Result<Value, String> {
	let config = &custom_config
		.as_ref()
		.map(|options| Config::from_config_options(&app.config(), &options))
		.unwrap_or_else(|| Ok(state.inner().lock()?.config.clone()))
		.map_err(|err| err.to_string())?;

	let new_settings = settings::set(config, key, value).map_err(|err| err.to_string())?;
	if let None = custom_config {
		state
			.inner()
			.lock()
			.map_err(|err| err.to_string())?
			.settings = new_settings.clone();
	}

	Ok(new_settings)
}

#[tauri::command]
pub fn set_cache<R: Runtime>(
	_app: AppHandle<R>,
	state: State<'_, PluginState>,
	key: &str,
	new_value: Value,
) -> Result<Value, String> {
	let settings_cache = &state
		.inner()
		.lock()
		.map_err(|err| err.to_string())?
		.settings;

	let new_settings = set_dot_notation(
		settings_cache,
		key,
		to_value(new_value).map_err(|err| err.to_string())?,
	)
	.map_err(|err| err.to_string())?;

	Ok(new_settings)
}

#[tauri::command]
pub fn cache_to_file<R: Runtime>(
	_app: AppHandle<R>,
	state: State<'_, PluginState>,
) -> Result<(), String> {
	let state = &state.inner().lock().map_err(|err| err.to_string())?;

	save_settings_json(&state.settings, &state.config).map_err(|err| err.to_string())?;

	Ok(())
}

#[tauri::command]
pub fn file_to_cache<R: Runtime>(
	_app: AppHandle<R>,
	state: State<'_, PluginState>,
) -> Result<(), String> {
	let state = &mut state.inner().lock().map_err(|err| err.to_string())?;

	let (settings_json, _, _) = load_settings_json(&state.config).map_err(|err| err.to_string())?;
	let settings: Value = serde_json::from_str(&settings_json).map_err(|err| err.to_string())?;

	state.settings = settings;

	Ok(())
}
