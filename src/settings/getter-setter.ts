import { getSettings, saveSettings } from '../fs/load-save';

/**
 * Checks whether a key exists in the settings.
 * @param key The key for the setting
 */
export async function has
  <SettingsSchema extends {} = any>
  (key: string | number | symbol): Promise<boolean>
{
  return (key in (await getSettings<SettingsSchema>()).settings);
}

/**
 * Get the value of a particular setting.
 * @param key The key for the setting
 * @returns The value
 */
export async function get
  <SettingsSchema extends {} = any, K  extends keyof SettingsSchema = keyof SettingsSchema>
  (key: K): Promise<SettingsSchema[K]>
{
  if (await has<SettingsSchema>(key)) return (await getSettings<SettingsSchema>()).settings[key];
  else throw 'Error: key does not exist.'
}

/**
 * Sets the value of a particular setting
 * @param key The key for the setting
 * @param value The new value
 * @returns The entire settings object
 */
export async function set
<SettingsSchema extends {} = any, K extends keyof SettingsSchema = keyof SettingsSchema>
(key: K, value: SettingsSchema[K]): Promise<SettingsSchema>
{
  const settings = await getSettings<SettingsSchema>();

  settings.settings[key] = value;
  await saveSettings<SettingsSchema>(settings.settings, settings.path);

  return settings.settings;
}
