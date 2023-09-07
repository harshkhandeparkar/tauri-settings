import { IConfigOptions } from '../utils/config';

import { has, get, set, hasCache, getCache, setCache } from './getter-setter';
import { add_config, cache_to_file } from '../utils/handlers';
import { Path, PathValue } from '../utils/dot-notation';

export class SettingsManager<SettingsSchema extends {} = any> {
	/**
	 * The default values for the settings
	 */
	default: SettingsSchema;
	/**
	 * Configuration for the settings manager
	 */
	config: IConfigOptions;
	/** @internal */
	configId: number = 0;

	constructor(defaultSettings: SettingsSchema, config?: IConfigOptions) {
		this.default = { ...defaultSettings };
		this.config = config ?? {};
	}

	/**
	 * Initializes a settings file with the defaults. If settings exist, load them.
	 * @returns The entire settings object
	 */
	async initialize(): Promise<void> {
		let configId = await add_config(this.config, this.default);

		this.configId = configId;
	}

	/**
	 * @internal
	 */
	protected async saveSettings() {
		await cache_to_file(this.configId);
	}

	/**
	 * Checks whether a key exists in the settings cache.
	 * @param key The key for the setting. Key supports dot notation. See https://github.com/harshkhandeparkar/tauri-settings#dot-notation.
	 */
	async hasCache<K extends Path<SettingsSchema>>(key: K): Promise<boolean> {
		return hasCache<SettingsSchema, K>(key, this.configId);
	}

	/**
	 * Sets the value of a setting from the cache.
	 * @param key The key for the setting. Key supports dot notation. See https://github.com/harshkhandeparkar/tauri-settings#dot-notation.
	 * @returns The value of the setting
	 */
	async getCache<K extends Path<SettingsSchema>>(key: K): Promise<PathValue<SettingsSchema, K>> {
		return getCache<SettingsSchema, K>(key, this.configId);
	}

	/**
	 * Sets the value for a setting. Only updates cache.
	 * @param key The key for the setting. Key supports dot notation. See https://github.com/harshkhandeparkar/tauri-settings#dot-notation.
	 * @param value The new value for the setting
	 */
	async setCache<K extends Path<SettingsSchema>, V extends PathValue<SettingsSchema, K>>(key: K, value: V): Promise<void> {
		await setCache<SettingsSchema, K>(key, value, this.configId);
	}

	/**
	 * Checks whether a key exists in the settings file.
	 * @param key The key for the setting. Key supports dot notation. See https://github.com/harshkhandeparkar/tauri-settings#dot-notation.
	 */
	async has<K extends Path<SettingsSchema>>(key: K): Promise<boolean> {
		return has<SettingsSchema, K>(key, this.configId);
	}

	/**
	 * Gets the value of a setting directly from the storage. Also updates cache.
	 * @param key The key for the setting. Key supports dot notation. See https://github.com/harshkhandeparkar/tauri-settings#dot-notation.
	 * @returns The value of the setting
	 */
	async get<K extends Path<SettingsSchema>>(key: K): Promise<PathValue<SettingsSchema, K>> {
		return get<SettingsSchema, K>(key, this.configId);
	}

	/**
	 * Sets the value for a setting directly to the storage. Also updates cache.
	 * @param key The key for the setting. Key supports dot notation. See https://github.com/harshkhandeparkar/tauri-settings#dot-notation.
	 * @param value The new value for the setting
	 */
	async set<K extends Path<SettingsSchema>, V extends PathValue<SettingsSchema, K>>(key: K, value: V): Promise<void> {
		await set<SettingsSchema, K>(key, value, this.configId);
	}

	/**
	 * Saves the current settings cache to the storage.
	 * @returns The entire settings object
	 */
	async syncCache() {
		await this.saveSettings();
	}
}
