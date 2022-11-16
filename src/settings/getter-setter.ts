import { ConfigOptions } from '../config/config';
import { getSettings, saveSettings } from '../fs/load-save';
import { getDotNotation, setDotNotation } from '../utils/dot-notation';
import type { Path, PathValue } from '../types/dot-notation';

/**
 * Checks whether a key exists in the settings.
 * @param key The key for the setting
 */
export async function has<
  SettingsSchema,
  K extends Path<SettingsSchema> = Path<SettingsSchema>
> (key: K, options: ConfigOptions = {}): Promise<boolean>
{
  try {
    const settings = (await getSettings<SettingsSchema>(options)).settings;
    const value = getDotNotation(settings, key);

    return value !== null;
  }
  catch (e) {
    throw e;
  }
}

/**
 * Get the value of a particular setting.
 * @param key The key for the setting
 * @returns The value of the setting
 */
export async function get<
  SettingsSchema,
  K extends Path<SettingsSchema> = Path<SettingsSchema>
> (key: K, options: ConfigOptions = {}): Promise<PathValue<SettingsSchema, K>>
{
  if (!await has<SettingsSchema, K>(key)) throw 'Error: key does not exist';

  try {
    const settings = (await getSettings<SettingsSchema>(options)).settings;
    return getDotNotation<SettingsSchema, K>(settings, key);
  }
  catch (e) {
    throw e;
  }
}

/**
 * Sets the value of a particular setting
 * @param key The key for the setting
 * @param value The new value
 * @returns The entire settings object
 */
export async function set<
  SettingsSchema,
  K extends Path<SettingsSchema> = Path<SettingsSchema>,
  V extends PathValue<SettingsSchema, K> = PathValue<SettingsSchema, K>
> (key: K, value: V, options: ConfigOptions = {}): Promise<SettingsSchema>
{
  if (!await has<SettingsSchema, K>(key)) throw 'Error: key does not exist';

  try {
    const settings = await getSettings<SettingsSchema>(options);
    setDotNotation<SettingsSchema, K>(settings.settings, key, value);

    await saveSettings<SettingsSchema>(settings.settings, settings.path, options);

    return settings.settings;
  }
  catch (e) {
    throw e;
  }
}
