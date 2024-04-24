import { ISettingsFile } from '../utils/settings_file';

import { has, get, set } from './getter-setter';
import { Path, PathValue } from '../utils/dot-notation';
import { add_settings_file } from '../utils/handlers';

export class SettingsManager<SettingsSchema extends {} = any> {
	/**
	 * The default values for the settings
	 */
	default: SettingsSchema;
	/**
	 * Configuration for the settings manager
	 */
	settings_file_options: ISettingsFile;
	/** @internal */
	fileId: number = 0;

	constructor(defaultSettings: SettingsSchema, settings_file_options?: ISettingsFile) {
		this.default = { ...defaultSettings };
		this.settings_file_options = settings_file_options ?? {};
	}

	/**
	 * Initializes a settings file with the defaults. If settings exist, load them.
	 * @returns The entire settings object
	 */
	async initialize(): Promise<void> {
		this.fileId = await add_settings_file(this.settings_file_options);
	}

	/**
	 * Checks whether a key exists in the settings file.
	 * @param key The key for the setting. Key supports dot notation. See https://github.com/harshkhandeparkar/tauri-settings#dot-notation.
	 */
	async has<K extends Path<SettingsSchema>>(key: K): Promise<boolean> {
		return has<SettingsSchema, K>(key, this.fileId);
	}

	/**
	 * Gets the value of a setting directly from the storage. Also updates cache.
	 * @param key The key for the setting. Key supports dot notation. See https://github.com/harshkhandeparkar/tauri-settings#dot-notation.
	 * @returns The value of the setting
	 */
	async get<K extends Path<SettingsSchema>>(key: K): Promise<PathValue<SettingsSchema, K>> {
		return get<SettingsSchema, K>(key, this.fileId);
	}

	/**
	 * Sets the value for a setting directly to the storage. Also updates cache.
	 * @param key The key for the setting. Key supports dot notation. See https://github.com/harshkhandeparkar/tauri-settings#dot-notation.
	 * @param value The new value for the setting
	 */
	async set<K extends Path<SettingsSchema>, V extends PathValue<SettingsSchema, K>>(key: K, value: V): Promise<void> {
		await set<SettingsSchema, K>(key, value, this.fileId);
	}
}
