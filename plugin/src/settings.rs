use crate::dot_notation::{self, set_dot_notation};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{from_value, to_value, Value};
use std::{error::Error, fs, path::PathBuf};

#[derive(Debug, Clone)]
/// Struct representing one settings JSON file.
pub struct SettingsFile {
	/// Path to the settings file
	pub file_path: PathBuf,
	/// Whether to prettify the JSON output. (Default: `false`)
	pub prettify: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsFileOptions {
	/// Path to the settings file relative to the scope directory.
	pub scoped_file_path: String,
	/// Whether to prettify the JSON before storing in the settings file. (Default: `false`)
	pub prettify: Option<bool>,
	/// The default values to write to the settings file when creating the file for the first time. (Default: an empty object)
	pub default_settings: Option<Value>,
}

impl SettingsFile {
	pub fn new(
		file_path: PathBuf,
		prettify: Option<bool>,
		default_settings: Option<Value>,
	) -> Result<Self, Box<dyn Error>> {
		let settings_file = Self {
			file_path,
			prettify: prettify.unwrap_or(false),
		};

		settings_file.ensure_settings_file(default_settings.unwrap_or(Value::Null))?;

		Ok(settings_file)
	}

	pub fn has(&self, key: &str) -> Result<bool, Box<dyn Error>> {
		let settings = self.load_settings()?;

		dot_notation::exists_dot_notation(&settings, key)
	}

	pub fn get<V: DeserializeOwned>(&self, key: &str) -> Result<V, Box<dyn Error>> {
		let settings = self.load_settings()?;

		let value: Value = dot_notation::get_dot_notation(&settings, key)?;

		Ok(from_value(value)?)
	}

	pub fn set<V: Serialize>(&self, key: &str, new_value: V) -> Result<(), Box<dyn Error>> {
		let mut settings = self.load_settings()?;

		set_dot_notation(&mut settings, key, to_value(new_value)?)?;
		self.save_settings(&settings)?;

		Ok(())
	}

	fn ensure_settings_file(&self, default_settings: Value) -> Result<bool, Box<dyn Error>> {
		if !self.file_path.exists() {
			self.save_settings(&default_settings)?;
			return Ok(true);
		}

		Ok(false)
	}

	fn load_settings(&self) -> Result<Value, Box<dyn Error>> {
		let settings_json = fs::read_to_string(&self.file_path)?;

		let settings: serde_json::Value = serde_json::from_str(&settings_json)?;

		Ok(settings)
	}

	fn save_settings<T: ?Sized + serde::Serialize>(
		&self,
		settings: &T,
	) -> Result<(), Box<dyn Error>> {
		let settings_json = if self.prettify {
			serde_json::to_string_pretty(&settings)?
		} else {
			serde_json::to_string(&settings)?
		};

		fs::write(&self.file_path, settings_json)?;
		Ok(())
	}
}
