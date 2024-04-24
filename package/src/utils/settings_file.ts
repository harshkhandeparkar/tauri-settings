export interface ISettingsFileOptions {
	/**
	 * The path to the settings file relative to the scope of the plugin.
	 * E.g. `settings.json` or `preferences/user.json`.
	 */
	scoped_file_path: string;
	/**
	 * Whether or not to prettify the JSON settings data before storing to the file.
	 * Default: false
	 */
	prettify?: boolean;
	default_settings?: any;
}
