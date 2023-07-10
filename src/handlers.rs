use serde_json::Value;
use tauri::{AppHandle, Runtime, State};

use crate::{config::Config, fs::load_settings_json};

#[tauri::command]
pub fn get<R: Runtime>(
	_app: AppHandle<R>,
	state: State<'_, Config>,
	key: &str
) -> Result<Value, String> {
	let config = state.inner();
	let settings_json = load_settings_json(config).map_err(|err| err.to_string())?;

	let settings: Value = serde_json::from_str(&settings_json).map_err(|err| err.to_string())?;

	let result = settings[key].clone();

	Ok(result)
}