use config::Config;
use tauri::{
	plugin::{Builder, TauriPlugin}, Manager, Runtime
};

mod config;
mod fs;
mod handlers;

pub fn init<R: Runtime>() -> TauriPlugin<R> {
	Builder::new("settings")
		.invoke_handler(tauri::generate_handler![handlers::has, handlers::get, handlers::set])
		.setup(|app| {
			let plugin_state = Config::new(&app.config(), None, None, None, None)?;

			app.manage(plugin_state);
			Ok(())
		})
		.build()
}
