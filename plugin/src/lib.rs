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
//!     .plugin(tauri_plugin_settings::init(None, None));
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
//! ```

mod config;
mod dot_notation;
mod handlers;
mod settings;
#[cfg(test)]
mod test;

use config::PluginConfig;
pub use config::PluginConfigOptions;
pub use settings::{SettingsFile, SettingsFileOptions};
use std::{collections::HashMap, error::Error, path::PathBuf, sync::Mutex};
use tauri::{
	api::path,
	plugin::{Builder, TauriPlugin},
	Manager, Runtime,
};

pub(crate) struct PluginStateData {
	plugin_config: config::PluginConfig,
	last_file_id: usize,
	settings_files: HashMap<usize, settings::SettingsFile>,
}

impl PluginStateData {
	pub(crate) fn add_settings_file(&mut self, settings_file: SettingsFile) -> usize {
		self.last_file_id += 1;
		self.settings_files.insert(self.last_file_id, settings_file);

		self.last_file_id
	}

	pub(crate) fn get_settings_file_mut(
		&mut self,
		id: usize,
	) -> Result<&mut SettingsFile, Box<dyn Error>> {
		self.settings_files
			.get_mut(&id)
			.ok_or(format!("Error: Settings file with ID {} does not exist.", id).into())
	}

	pub(crate) fn get_settings_file(&self, id: usize) -> Result<&SettingsFile, Box<dyn Error>> {
		self.settings_files
			.get(&id)
			.ok_or(format!("Error: Settings file with ID {} does not exist.", id).into())
	}

	pub(crate) fn find_settings_file(&self, file_path: PathBuf) -> Option<usize> {
		self.settings_files
			.iter()
			.find(|settings_file| settings_file.1.file_path == file_path)
			.map(|(&id, _)| id)
	}

	pub(crate) fn new(
		plugin_config: PluginConfig,
		initial_settings_files: Vec<SettingsFile>,
	) -> Result<PluginStateData, Box<dyn Error>> {
		let mut settings_files: HashMap<usize, SettingsFile> = HashMap::new();

		if plugin_config.files_limit != 0
			&& initial_settings_files.len() > plugin_config.files_limit
		{
			return Err(format!(
				"Error: Initial settings files ({}) more than the limit ({}).",
				initial_settings_files.len(),
				plugin_config.files_limit
			)
			.into());
		}

		for (i, settings_file) in initial_settings_files.iter().enumerate() {
			settings_files.insert(i, settings_file.clone());
		}

		Ok(PluginStateData {
			last_file_id: settings_files.len() - 1,
			settings_files,
			plugin_config,
		})
	}
}

pub(crate) type PluginState = Mutex<PluginStateData>;

/// Initializes the plugin.
///
/// ### Examples
/// ```no_run
/// tauri::Builder::default()
///     .plugin(tauri_plugin_settings::init(None, None));
/// ```
///
/// ```no_run
/// use tauri_plugin_settings::PluginConfigOptions;
///
/// // Allows adding new files from the frontend with a limit of 5 files.
/// let config = PluginConfigOptions::new(None, Some(5), Some(true));
///
/// tauri::Builder::default()
///     .plugin(tauri_plugin_settings::init(Some(config), None));
/// ```
pub fn init<R: Runtime>(
	plugin_config: Option<PluginConfigOptions>,
	initial_settings_files: Option<Vec<SettingsFile>>,
) -> TauriPlugin<R> {
	Builder::new("settings")
		.invoke_handler(tauri::generate_handler![
			handlers::has,
			handlers::get,
			handlers::set,
			handlers::add_settings_file,
			handlers::get_settings_file_id
		])
		.setup(move |app| {
			let app_config = app.config();
			let plugin_config = if let Some(plugin_config) = plugin_config {
				PluginConfig::from_options(&app_config, &plugin_config)?
			} else {
				PluginConfig::default(&app_config)?
			};

			let initial_settings_files =
				if let Some(initial_settings_files) = initial_settings_files {
					initial_settings_files
				} else {
					let app_config_dir = path::app_config_dir(&app_config)
						.ok_or("Error reading the app config directory.")?;
					let settings_file_path = app_config_dir.join("settings.json");

					vec![SettingsFile::new(settings_file_path, None, None).unwrap()]
				};

			app.manage::<PluginState>(Mutex::new(PluginStateData::new(
				plugin_config,
				initial_settings_files,
			)?));
			Ok(())
		})
		.build()
}
