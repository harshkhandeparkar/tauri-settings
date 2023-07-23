use serde_json::Value;

pub fn get_dot_notation(settings: &Value, path: String) -> Value {
	let keys = path.split('.');

	let value = settings.clone();

	for key in keys {
		let value = value[key];
	}

	value
}

pub fn set_dot_notation(settings: Value, path: String, new_value: Value) -> Value {
	let keys = path.split('.');

	let mut new_settings = settings.clone();
	let mut traverse = &mut new_settings;

	for key in keys {
		traverse = &mut traverse[key];
	}

	*traverse = new_value;

	new_settings
}