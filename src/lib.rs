use config::Config;
use tauri::{
	plugin::{Builder, TauriPlugin}, Manager, Runtime,
};

mod config;
mod fs;
mod handlers;

pub fn init<R: Runtime>() -> TauriPlugin<R> {
	Builder::new("settings")
		.invoke_handler(tauri::generate_handler![handlers::get])
		.setup(|app| {
			app.manage(Config::new(&app.config(), None, None, None, None));
			Ok(())
		})
		.build()
}
