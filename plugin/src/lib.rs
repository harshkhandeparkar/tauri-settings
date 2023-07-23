use config::Config;
use tauri::{
	plugin::{Builder, TauriPlugin},
	Manager, Runtime,
};

mod config;
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
