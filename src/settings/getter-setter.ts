import { ConfigOptions } from '../config/config';
import { getSettings, saveSettings } from '../fs/load-save';
import { getDotNotation, setDotNotation } from '../utils/dot-notation';
import type { Path, PathValue } from '../types/dot-notation';

/**
 * Get the value of a particular setting.
 * @param key The key for the setting
 * @returns The value or undefined if the key does not exist
 */
export async function get
  <SettingsSchema, K extends Path<SettingsSchema>>
  (key: K, options: ConfigOptions = {}): Promise<PathValue<SettingsSchema, K> | undefined> {
  if (typeof key !== 'string') throw 'Error: key must be a string';
  try {
    const settings = (await getSettings<SettingsSchema>(options)).settings;
    return getDotNotation(settings, key);
  } catch (error) {
    return undefined;
  }
}

/**
 * Sets the value of a particular setting
 * @param key The key for the setting
 * @param value The new value
 * @returns The entire settings object
 */
export async function set
  <SettingsSchema, K extends Path<SettingsSchema>, V extends PathValue<SettingsSchema, K>>
  (key: K, value: V, options: ConfigOptions = {}): Promise<SettingsSchema> {
  if (typeof key !== 'string') throw 'Error: key must be a string';

  const settings = await getSettings<SettingsSchema>(options);
  setDotNotation(settings.settings, key, value);

  await saveSettings<SettingsSchema>(settings.settings, settings.path, options);

  return settings.settings;
}
