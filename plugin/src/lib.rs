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
mod handlers;
pub mod settings;
#[cfg(test)]
mod test;

use config::{PluginConfig, PluginConfigOptions};
use settings::SettingsFile;
use std::{collections::HashMap, error::Error, sync::Mutex};
use tauri::{
	api::path, plugin::{Builder, TauriPlugin}, Manager, Runtime
};

pub(crate) struct PluginStateData {
	plugin_config: config::PluginConfig,
	last_file_id: u32,
	settings_files: HashMap<u32, settings::SettingsFile>,
}

impl PluginStateData {
	pub(crate) fn add_settings_file(&mut self, settings_file: SettingsFile) -> u32 {
		self.last_file_id += 1;
		self.settings_files.insert(self.last_file_id, settings_file);

		self.last_file_id
	}

	pub(crate) fn get_settings_file_mut(
		&mut self,
		id: u32,
	) -> Result<&mut SettingsFile, Box<dyn Error>> {
		self.settings_files
			.get_mut(&id)
			.ok_or("Error: Config does not exist.".into())
	}

	pub(crate) fn get_settings_file(&self, id: u32) -> Result<&SettingsFile, Box<dyn Error>> {
		self.settings_files
			.get(&id)
			.ok_or("Error: Config does not exist.".into())
	}

	pub(crate) fn new(
		plugin_config: PluginConfig,
		initial_settings_file: SettingsFile,
	) -> PluginStateData {
		let mut settings_files: HashMap<u32, SettingsFile> = HashMap::new();
		settings_files.insert(0, initial_settings_file);

		PluginStateData {
			last_file_id: 0,
			settings_files,
			plugin_config,
		}
	}
}

pub(crate) type PluginState = Mutex<PluginStateData>;

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
pub fn init<R: Runtime>(
	plugin_config: PluginConfigOptions,
	initial_settings_file: Option<SettingsFile>,
) -> TauriPlugin<R> {
	Builder::new("settings")
		.invoke_handler(tauri::generate_handler![
			handlers::has,
			handlers::get,
			handlers::set
		])
		.setup(|app| {
			let app_config = app.config();
			// TODO: BETTER HANDLE ERRORS
			let plugin_config = PluginConfig::from_options(&app_config, &plugin_config).unwrap();

			let initial_settings_file = if let Some(initial_settings_file) = initial_settings_file {
				initial_settings_file
			} else {
				let app_config_dir = path::app_config_dir(&app_config).unwrap();
				let settings_file_path = app_config_dir.join("settings.json");

				SettingsFile::new(settings_file_path, None).unwrap()
			};

			app.manage::<PluginState>(
				Mutex::new(
					PluginStateData::new(plugin_config, initial_settings_file)
				)
			);
			Ok(())
		})
		.build()
}
