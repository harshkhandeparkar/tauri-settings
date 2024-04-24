export interface ISettingsFile {
	/**
	 * The name of the file in which the settings should be saved.
	 * The filename should be without any extension.
	 * Default: 'settings'
	 */
	file_path?: string;
	/**
	 * Whether or not to prettify the JSON settings data before storing to the file.
	 * Default: false
	 */
	prettify?: boolean;
}
