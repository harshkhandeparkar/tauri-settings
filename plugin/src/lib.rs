use config::Config;
use tauri::{
	plugin::{Builder, TauriPlugin}, Manager, Runtime
};

mod config;
mod fs;
mod handlers;
mod dot_notation;
#[cfg(test)]
mod test;

pub fn init<R: Runtime>(config: Option<Config>) -> TauriPlugin<R> {
	Builder::new("settings")
		.invoke_handler(tauri::generate_handler![handlers::has, handlers::get, handlers::set])
		.setup(|app| {
			let plugin_state = config.unwrap_or(Config::new(&app.config(), None, None, None, None)?);

			app.manage(plugin_state);
			Ok(())
		})
		.build()
}
