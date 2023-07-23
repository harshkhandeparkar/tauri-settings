use std::error::Error;

use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_value, to_value, Value};

pub fn get_dot_notation<S, T>(settings: &S, path: String) -> Result<T, Box<dyn Error>>
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

pub fn set_dot_notation<S, T>(settings: S, path: String, new_value: Value) -> Result<T, Box<dyn Error>>
where
	S: Sized + Serialize + DeserializeOwned + Default,
	T: Sized + Serialize + DeserializeOwned + Default,
{
	let keys = path.split('.');

	let mut new_settings = to_value(settings)?;
	let mut traverse = &mut new_settings;

	for key in keys {
		traverse = &mut traverse[key];
	}

	*traverse = new_value;

	let new_settings: T = from_value(new_settings)?;
	Ok(new_settings)
}
