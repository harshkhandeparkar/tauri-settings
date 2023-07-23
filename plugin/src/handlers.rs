use serde_json::Value;
use tauri::{AppHandle, Runtime, State};

use crate::{
	config::Config,
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
