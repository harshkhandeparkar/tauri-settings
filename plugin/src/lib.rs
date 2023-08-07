#![doc(
	html_favicon_url = "https://raw.githubusercontent.com/harshkhandeparkar/tauri-settings/plugin/img/tauri-settings-logo-circular.png"
)]
#![doc(
	html_logo_url = "https://raw.githubusercontent.com/harshkhandeparkar/tauri-settings/plugin/img/tauri-settings-logo-circular.png"
)]
#![doc(html_no_source)]

//! ### Tauri Settings
//!  A user settings manager for [Tauri](https://tauri.app) inspired by `electron-settings`.
//!
//! This crate exports a Tauri [plugin](https://tauri.app/v1/guides/features/plugin) through the [`init`] function that can be used a backend for the accompanying Javascript library [`tauri-settings`](https://www.npmjs.com/package/tauri-settings) and also exports a [`settings`] module which can be used to load/save settings from Rust.
//!
//! #### Getting Started
//! ##### Using the Plugin
//! Initialize the Tauri plugin by using the [`init`] function in the `src-tauri/src/main.rs` file.
//! ```no_run
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
//! ```no_run
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
//! let theme: String = get(&config, "theme").unwrap();
//!
//! set(&config, "open_fullscreen", true).unwrap();
//! ```

mod config;
mod dot_notation;
mod fs;
mod handlers;
pub mod settings;
#[cfg(test)]
mod test;

use serde_json::Value;
use std::sync::Mutex;
use tauri::{
	plugin::{Builder, TauriPlugin},
	Manager, Runtime,
};

pub use config::{Config, ConfigOptions};

pub struct PluginStateConfig {
	config: Config,
	settings: Value,
}
pub(crate) type PluginState = Mutex<PluginStateConfig>;

/// Initializes the plugin.
///
/// ### Examples
/// ```no_run
/// tauri::Builder::default()
///     .plugin(tauri_plugin_settings::init(None));
/// ```
///
/// ```no_run
/// use tauri_plugin_settings::ConfigOptions;
///
/// let config = ConfigOptions::new(Some("preferences.json".into()), None, Some(true.into()));
///
/// tauri::Builder::default()
///     .plugin(tauri_plugin_settings::init(Some(config)));
/// ```
pub fn init<R: Runtime>(custom_config: Option<ConfigOptions>) -> TauriPlugin<R> {
	Builder::new("settings")
		.invoke_handler(tauri::generate_handler![
			handlers::has,
			handlers::get,
			handlers::set,
			handlers::read_settings,
			handlers::overwrite_settings
		])
		.setup(|app| {
			let config = custom_config
				.map(|options| Config::from_config_options(&app.config(), &options))
				.unwrap_or_else(|| Config::new(&app.config(), None, None, None))?;

			let (initial_settings_json, settings_file_path, was_created) =
				fs::load_settings_json(&config).map_err(|err| err.to_string())?;

			let initial_settings: Value =
				serde_json::from_str(&initial_settings_json).map_err(|err| err.to_string())?;

			app.manage::<PluginState>(Mutex::new(PluginStateConfig {
				config,
				settings: initial_settings,
			}));
			Ok(())
		})
		.build()
}
