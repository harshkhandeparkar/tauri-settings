//! Helper functions for getting and setting values using dot notation.

use serde_json::Value;
use std::error::Error;

/// Checks if a given key (in dot notation) exists in the settings.
///
/// `path` is the dot notation for a key in the settings. (e.g.: `preferences.theme`)
///
/// ### Examples
/// ```ignore
/// if exists_dot_notation(&settings, "preferences.theme").unwrap() {
///     println!("Key exists!");
/// }
/// ```
pub fn exists_dot_notation(settings: &Value, path: &str) -> Result<bool, Box<dyn Error>> {
	let keys = path.split('.');

	let mut traverse = settings;

	for key in keys {
		if traverse.is_null() {
			return Ok(false);
		}

		traverse = &traverse[key];
	}

	Ok(true)
}

/// Gets the value in `settings` corresponding to the dot notation for a key.
///
/// `path` is the dot notation for a key in the settings. (e.g.: `preferences.theme`)
///
/// ### Examples
/// ```ignore
///
/// let theme: &str = from_value(get_dot_notation(&settings, "preferences.theme")?);
/// if theme == "dark" {
///     // do something
/// }
/// ```
pub fn get_dot_notation(settings: &Value, path: &str) -> Result<Value, Box<dyn Error>> {
	let keys = path.split('.');

	let mut traverse = settings;

	for key in keys {
		traverse = &traverse[key];
	}

	Ok(traverse.clone())
}

/// Recursively sets the value in `settings` corresponding to the dot notation for a key.
///
/// `path` is the dot notation for a key in the settings. (e.g.: `preferences.theme`)
///
/// ### Examples
/// ```ignore
/// set_dot_notation(&settings, "preferences.theme", "dark").unwrap();
/// ```
pub fn set_dot_notation(
	settings: &mut Value,
	path: &str,
	new_value: Value,
) -> Result<(), Box<dyn Error>> {
	let keys = path.split('.');

	let mut traverse = settings;

	for key in keys {
		traverse = &mut traverse[key];
	}

	*traverse = new_value;

	Ok(())
}
