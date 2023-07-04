use tauri::{
    plugin::{Builder, TauriPlugin}, Manager, Runtime,
};

use std::{collections::HashMap, sync::Mutex};

mod config;
mod fs;

#[derive(Default)]
struct MyState(Mutex<HashMap<String, String>>);

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("settings")
        .invoke_handler(tauri::generate_handler![])
        .setup(|app| {
            app.manage(MyState::default());
            Ok(())
        })
        .build()
}
