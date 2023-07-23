use serde_json::Value;
use crate::dot_notation::{get_dot_notation, set_dot_notation};

fn get_dummy_data() -> Value {
	serde_json::from_str(r#"
	{
		"name": "John Doe",
		"age": 43,
		"preferences": {
			"theme": "dark",
			"open_on_start": true
		}
	}"#).unwrap()
}

fn get_updated_dummy_data() -> Value {
	serde_json::from_str(r#"
	{
		"name": "John Lark",
		"age": 40,
		"preferences": {
			"theme": "light",
			"open_on_start": 0,
			"fullscreen": true,
			"run_out_of_names": {
				"another_setting": 12
			}
		},
		"recently_opened": ["file1", "file2"],
		"test": {
			"test_path": "ok"
		}
	}"#).unwrap()
}

#[test]
fn get_dot_notation_works() {
	let data: Value = get_dummy_data();

	let get_name = get_dot_notation(&data, "name".into());
	let get_age = get_dot_notation(&data, "age".into());
	let get_theme = get_dot_notation(&data, "preferences.theme".into());
	let get_startup = get_dot_notation(&data, "preferences.open_on_start".into());
	let get_null = get_dot_notation(&data, "preferences.fullscreen".into());

	assert_eq!(get_name, "John Doe");
	assert_eq!(get_age, 43);
	assert_eq!(get_theme, "dark");
	assert_eq!(get_startup, true);
	assert_eq!(get_null, Value::Null);
}

#[test]
fn set_dot_notation_works() {
	let data: Value = get_dummy_data();
	let updated_data: Value = get_updated_dummy_data();

	let data = set_dot_notation(data, "name".into(), "John Lark".into());
	let data = set_dot_notation(data, "age".into(), 40.into());
	let data = set_dot_notation(data, "preferences.theme".into(), "light".into());
	let data = set_dot_notation(data, "preferences.open_on_start".into(), 0.into());
	let data = set_dot_notation(data, "preferences.fullscreen".into(), true.into());
	let data = set_dot_notation(data, "preferences.run_out_of_names.another_setting".into(), 12.into());
	let data = set_dot_notation(data, "recently_opened".into(), vec!["file1", "file2"].into());
	let data = set_dot_notation(data, "test.test_path".into(), "ok".into());


	assert_eq!(data, updated_data);
}