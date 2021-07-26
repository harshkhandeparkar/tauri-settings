import { ConfigOptions } from '../config/config';
import { STATUS } from '../fs/ensure-settings-file';

import { getSettings, saveSettings } from '../fs/load-save';
import { has, get, set } from '../settings/getter-setter';

export class SettingsManager<SettingsSchema extends {} = any> {
  /**
   * @internal
   */
  settings: SettingsSchema;
  /**
   * The default values for the settings
   */
  default: SettingsSchema;
  /**
   * @internal
   */
  path: string;
  options: ConfigOptions;

  constructor(defaultSettings: SettingsSchema, options: ConfigOptions = {}) {
    this.default = { ...defaultSettings };
    this.options = { ...options };
  }

  /**
   * Initializes a settings file with the defaults. If settings exist, load them.
   * @returns The entire settings object
   */
  async initialize(): Promise<SettingsSchema> {
    const currentSettings = await getSettings<SettingsSchema>(this.options);
    this.path = currentSettings.path;

    if (currentSettings.status === STATUS.FILE_CREATED) {
      this.settings = { ...this.default };
      await this.saveSettings();
    }

    return this.settings;
  }

  /**
   * @internal
   */
  protected async saveSettings() {
    await saveSettings<SettingsSchema>(this.settings, this.path, this.options);
  }

  /**
   * Checks whether a key exists in the settings cache.
   * @param key The key for the setting
   */
  hasCache(key: string | number | symbol): boolean {
    return key in this.settings;
  }

  /**
   * Sets the value of a setting from the cache.
   * @param key The key for the setting
   * @returns The value of the setting
   */
  getCache<K extends keyof SettingsSchema = keyof SettingsSchema>(key: K): SettingsSchema[K] {
    if (this.hasCache(key)) {
      return this.settings[key];
    }
    else throw 'Error: key does not exist';
  }

  /**
   * Sets the value for a setting. Only updates cache.
   * @param key The key for the setting
   * @param value The new value for the setting
   * @returns The entire settings object
   */
  setCache<K extends keyof SettingsSchema = keyof SettingsSchema>(key: K, value: SettingsSchema[K]): SettingsSchema[K] {
    this.settings[key] = value;

    return value;
  }

  /**
   * Checks whether a key exists in the settings directly from the storage.
   * @param key The key for the setting
   */
  async has(key: string | number | symbol): Promise<boolean> {
    return await has<SettingsSchema>(key, this.options);
  }

  /**
   * Gets the value of a setting directly from the storage. Also updates cache.
   * @param key The key for the setting
   * @returns The value of the setting
   */
  async get<K extends keyof SettingsSchema = keyof SettingsSchema>(key: K): Promise<SettingsSchema[K]> {
    if (await this.has(key)) {
      const value = await get<SettingsSchema, K>(key, this.options);

      // to also update cache
      this.setCache(key, value);

      return value;
    }
    else throw 'Error: key does not exist';
  }

  /**
   * Sets the value for a setting directly to the storage. Also updates cache.
   * @param key The key for the setting
   * @param value The new value for the setting
   * @returns The entire settings object
   */
  async set<K extends keyof SettingsSchema = keyof SettingsSchema>(key: K, value: SettingsSchema[K]): Promise<SettingsSchema> {
    // to also update cache
    this.setCache(key, value);

    return await set<SettingsSchema, K>(key, value, this.options);
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
