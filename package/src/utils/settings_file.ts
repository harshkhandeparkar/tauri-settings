export interface ISettingsFileOptions<SettingsSchema> {
	/**
	 * The settings file to use. Either an ID or the path to the settings file relative to the scope of the plugin.
	 * E.g. `settings.json` or `preferences/user.json`.
	 * Default: ID `0`.
	 */
	file?: string | number;
	/**
	 * Whether or not to prettify the JSON settings data before storing to the file.
	 * Default: false
	 */
	prettify?: boolean;
	default_settings: SettingsSchema;
}
