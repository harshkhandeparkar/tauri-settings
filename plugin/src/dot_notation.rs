//! Helper functions for getting and setting values using dot notation.

use std::error::Error;

use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_value, to_value};

/// Gets the value in `settings` corresponding to the dot notation for a key.
///
/// `path` is the dot notation for a key in the settings. (e.g.: `preferences.theme`)
///
/// ### Examples
/// ```ignore
/// let theme: &str = get_dot_notation(&settings, "preferences.theme");
/// if theme == "dark" {
/// 	// do something
/// }
/// ```
pub fn get_dot_notation<S, T>(settings: &S, path: &str) -> Result<T, Box<dyn Error>>
where
	S: Sized + Serialize + DeserializeOwned + Default,
	T: Sized + Serialize + DeserializeOwned + Default,
{
	let keys = path.split('.');

	let mut traverse = to_value(settings)?;

	for key in keys {
		traverse = traverse[key].clone();
	}

	let value: T = from_value(traverse)?;

	Ok(value)
}

/// Recursively sets the value in `settings` corresponding to the dot notation for a key.
///
/// `path` is the dot notation for a key in the settings. (e.g.: `preferences.theme`)
///
/// ### Examples
/// ```ignore
/// set_dot_notation(&settings, "preferences.theme", "dark");
/// ```
pub fn set_dot_notation<S, T, V>(
	settings: &S,
	path: &str,
	new_value: V,
) -> Result<T, Box<dyn Error>>
where
	S: Sized + Serialize + DeserializeOwned + Default,
	T: Sized + Serialize + DeserializeOwned + Default,
	V: Serialize,
{
	let keys = path.split('.');

	let mut new_settings = to_value(settings)?;
	let mut traverse = &mut new_settings;

	for key in keys {
		traverse = &mut traverse[key];
	}

	*traverse = to_value(new_value)?;

	let new_settings: T = from_value(new_settings)?;
	Ok(new_settings)
}
