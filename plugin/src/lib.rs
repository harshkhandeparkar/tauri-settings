//! ### Tauri Settings
//!  A user settings manager for [Tauri](https://tauri.app) inspired by `electron-settings`.
//!
//! This crate exports a Tauri [plugin](https://tauri.app/v1/guides/features/plugin) through the `init` function that can be used a backend for the accompanying Javascript library [`tauri-settings`](https://www.npmjs.com/package/tauri-settings) and also exports a `settings` module which can be used to load/save settings from Rust.
//!
//! #### Getting Started
//! ##### Using the Plugin
//! Initialize the Tauri plugin by using the `init` function in the `src-tauri/src/main.rs` file.
//! ```rust
//!	fn main() {
//! tauri::Builder::default()
//! 	.plugin(tauri_plugin_settings::init())
//! 	.run(tauri::generate_context!())
//! 	.expect("failed to run app");
//! }
//! ```
//!
//! Use the `tauri-settings` library in the frontend.
//! ```javascript
//! import { get } from 'tauri-settings';
//!
//! get('theme').then((theme) => {
//! 	console.log(`Changing theme to ${theme}.`);
//! 	// change the theme
//! })
//! ```
//! See the [README](https://github.com/harshkhandeparkar/tauri-settings#readme) for more information on how to install and use the `tauri-settings` library.
//!
//! ##### Using Tauri Settings Directly in Rust
//! ```rust
//! use tauri_plugin_settings::{settings::{get, set}, config::Config};
//! ```
//!
//! ```rust
//! // Where app is tauri::AppHandle
//! let config = Config::new(
//! 	&app.config(),
//! 	Some("user-settings.json"), // File in which the settings are saved
//! 	None, 						// Config directory
//! 	Some(true),					// Whether to prettify the JSON
//! 	None,						// Number of spaces in the prettified JSON
//! )
//!
//! // The returned value is a serde_json::value::Value
//! let theme = get(&config, "theme")?;
//! ```

use config::Config;
use tauri::{
	plugin::{Builder, TauriPlugin},
	Manager, Runtime,
};

pub mod config;
mod dot_notation;
mod fs;
mod handlers;
pub mod settings;
#[cfg(test)]
mod test;

pub fn init<R: Runtime>(config: Option<Config>) -> TauriPlugin<R> {
	Builder::new("settings")
		.invoke_handler(tauri::generate_handler![
			handlers::has,
			handlers::get,
			handlers::set,
			handlers::read_settings,
			handlers::overwrite_settings
		])
		.setup(|app| {
			let plugin_state =
				config.unwrap_or(Config::new(&app.config(), None, None, None, None)?);

			app.manage(plugin_state);
			Ok(())
		})
		.build()
}
