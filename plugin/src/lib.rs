//! ### Tauri Settings
//!  A user settings manager for [Tauri](https://tauri.app) inspired by `electron-settings`.
//!
//! This crate exports a Tauri [plugin](https://tauri.app/v1/guides/features/plugin) through the `init` function that can be used a backend for the accompanying Javascript library [`tauri-settings`](https://www.npmjs.com/package/tauri-settings) and also exports a `settings` module which can be used to load/save settings from Rust.
//!
//! #### Getting Started
//! ##### Using the Plugin
//! Initialize the Tauri plugin by using the `init` function in the `src-tauri/src/main.rs` file.
//! ```
//! tauri::Builder::default()
//!     .plugin(tauri_plugin_settings::init(None));
//! ```
//!
//! Use the `tauri-settings` library in the frontend.
//! ```javascript
//! import { get } from 'tauri-settings';
//!
//! get('theme').then((theme) => {
//!     console.log(`Changing theme to ${theme}.`);
//!     // change the theme
//! })
//! ```
//! See the [README](https://github.com/harshkhandeparkar/tauri-settings#readme) for more information on how to install and use the `tauri-settings` library.
//!
//! ##### Using Tauri Settings Directly in Rust
//! ```
//! use tauri_plugin_settings::{settings::{get, set}, Config};
//!
//! # let app_config = tauri::Config::default();
//! // Where app_config is tauri::Config
//! let config = Config::new(
//!     &app_config,
//!     Some("user-settings.json".into()), // File in which the settings are saved
//!     None, // Config directory
//!     Some(true), // Whether to prettify the JSON
//! ).unwrap();
//!
//! // The returned value is a serde_json::value::Value
//! let theme: String = get(&config, "theme").unwrap();
//!
//! set(&config, "open_fullscreen", true).unwrap();
//! ```

use tauri::{
	plugin::{Builder, TauriPlugin},
	Manager, Runtime,
};

pub use config::Config;

mod config;
mod dot_notation;
mod fs;
mod handlers;
pub mod settings;
#[cfg(test)]
mod test;

/// Initializes the plugin.
///
/// ### Examples
/// ```
/// tauri::Builder::default()
///     .plugin(tauri_plugin_settings::init(None));
/// ```
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
			let plugin_state = config.unwrap_or(Config::new(&app.config(), None, None, None)?);

			app.manage(plugin_state);
			Ok(())
		})
		.build()
}
