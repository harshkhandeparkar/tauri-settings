import { IConfigOptions } from '../utils/config';

import { getSettings, saveSettings } from './load-save';
import { get, set } from './getter-setter';
import { getDotNotation, setDotNotation, Path, PathValue } from '../utils/dot-notation';

export class SettingsManager<SettingsSchema extends {} = any> {
	/**
	 * @internal
	 */
	settings: SettingsSchema;
	/**
	 * The default values for the settings
	 */
	default: SettingsSchema;
	customConfig?: IConfigOptions;

	constructor(defaultSettings: SettingsSchema, customConfig?: IConfigOptions) {
		this.default = { ...defaultSettings };
		this.customConfig = customConfig
	}

	/**
	 * Initializes a settings file with the defaults. If settings exist, load them.
	 * @returns The entire settings object
	 */
	async initialize(): Promise<SettingsSchema> {
		const currentSettings = await getSettings<SettingsSchema>(this.customConfig);

		if (currentSettings.was_created) {
			this.settings = { ...this.default };
			await this.saveSettings();
		}
		else {
			this.settings = { ...this.default, ...currentSettings.settings };
		}

		return this.settings;
	}

	/**
	 * @internal
	 */
	protected async saveSettings() {
		await saveSettings<SettingsSchema>(this.settings, this.customConfig);
	}

	/**
	 * Checks whether a key exists in the settings cache.
	 * @param key The key for the setting. Key supports dot notation. See https://github.com/harshkhandeparkar/tauri-settings#dot-notation.
	 */
	hasCache<K extends Path<SettingsSchema>>(key: K): boolean {
		return getDotNotation(this.settings, key) !== null;
	}

	/**
	 * Sets the value of a setting from the cache.
	 * @param key The key for the setting. Key supports dot notation. See https://github.com/harshkhandeparkar/tauri-settings#dot-notation.
	 * @returns The value of the setting
	 */
	getCache<K extends Path<SettingsSchema>>(key: K): PathValue<SettingsSchema, K> {
		if (!this.hasCache(key)) throw 'Error: key does not exist';

		return getDotNotation<SettingsSchema, K>(this.settings, key) as PathValue<SettingsSchema, K>;
	}

	/**
	 * Sets the value for a setting. Only updates cache.
	 * @param key The key for the setting. Key supports dot notation. See https://github.com/harshkhandeparkar/tauri-settings#dot-notation.
	 * @param value The new value for the setting
	 * @returns The entire settings object
	 */
	setCache<K extends Path<SettingsSchema>, V extends PathValue<SettingsSchema, K>>(key: K, value: V): V {
		if (!this.hasCache(key)) throw 'Error: key does not exist';

		setDotNotation<SettingsSchema, K>(this.settings, key, value);

		return value;
	}

	/**
	 * Gets the value of a setting directly from the storage. Also updates cache.
	 * @param key The key for the setting. Key supports dot notation. See https://github.com/harshkhandeparkar/tauri-settings#dot-notation.
	 * @returns The value of the setting
	 */
	async get<K extends Path<SettingsSchema>>(key: K): Promise<PathValue<SettingsSchema, K>> {
		const value = await get<SettingsSchema, K>(key, this.customConfig);

		// to also update cache
		this.setCache(key, value);

		return value;
	}

	/**
	 * Sets the value for a setting directly to the storage. Also updates cache.
	 * @param key The key for the setting. Key supports dot notation. See https://github.com/harshkhandeparkar/tauri-settings#dot-notation.
	 * @param value The new value for the setting
	 * @returns The entire settings object
	 */
	async set<K extends Path<SettingsSchema>, V extends PathValue<SettingsSchema, K>>(key: K, value: V): Promise<SettingsSchema> {
		// to also update cache
		this.setCache(key, value);

		return await set<SettingsSchema, K, V>(key, value, this.customConfig);
	}

	/**
	 * Saves the current settings cache to the storage.
	 * @returns The entire settings object
	 */
	async syncCache(): Promise<SettingsSchema> {
		await this.saveSettings();

		return this.settings;
	}
}
