//! Tests for the crate.

use crate::dot_notation::{get_dot_notation, set_dot_notation};
use serde_json::Value;

fn get_dummy_data() -> Value {
	serde_json::from_str(
		r#"
	{
		"name": "John Doe",
		"age": 43,
		"preferences": {
			"theme": "dark",
			"open_on_start": true
		}
	}"#,
	)
	.unwrap()
}

fn get_updated_dummy_data() -> Value {
	serde_json::from_str(
		r#"
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
	}"#,
	)
	.unwrap()
}

#[test]
fn get_dot_notation_works() {
	let data: Value = get_dummy_data();

	let get_name: Value = get_dot_notation(&data, "name").unwrap();
	let get_age: Value = get_dot_notation(&data, "age").unwrap();
	let get_theme: Value = get_dot_notation(&data, "preferences.theme").unwrap();
	let get_startup: Value = get_dot_notation(&data, "preferences.open_on_start").unwrap();
	let get_null: Value = get_dot_notation(&data, "preferences.fullscreen").unwrap();

	assert_eq!(get_name, "John Doe");
	assert_eq!(get_age, 43);
	assert_eq!(get_theme, "dark");
	assert_eq!(get_startup, true);
	assert_eq!(get_null, Value::Null);
}

#[test]
fn set_dot_notation_works() {
	let mut data: Value = get_dummy_data();
	let updated_data: Value = get_updated_dummy_data();

	set_dot_notation(&mut data, "name", "John Lark".into()).unwrap();
	set_dot_notation(&mut data, "age", 40.into()).unwrap();
	set_dot_notation(&mut data, "preferences.theme", "light".into()).unwrap();
	set_dot_notation(&mut data, "preferences.open_on_start", 0.into()).unwrap();
	set_dot_notation(&mut data, "preferences.fullscreen", true.into()).unwrap();
	set_dot_notation(
		&mut data,
		"preferences.run_out_of_names.another_setting",
		12.into(),
	)
	.unwrap();
	set_dot_notation(&mut data, "recently_opened", vec!["file1", "file2"].into()).unwrap();
	set_dot_notation(&mut data, "test.test_path", "ok".into()).unwrap();

	assert_eq!(data, updated_data);
}
