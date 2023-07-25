export interface IConfigOptions {
	/**
	 * The name of the file in which the settings should be saved.
	 * The filename should be without any extension.
	 * Default: 'settings'
	 */
	file_name?: string;
	/**
	 * Path to the directory in which the settings file is to be created.
	 * Default: User's app config directory. See https://tauri.app/v1/api/js/path#appconfigdir
	 */
	directory?: string | null;
	/**
	 * Whether or not to prettify the JSON settings data before storing to the file.
	 * Default: false
	 */
	prettify?: boolean;
	/**
	 * The number of spaces to use when prettifying the data. Will only work if the 'prettify' option is enabled.
	 * Default: 2
	 */
	num_spaces?: number;
}
