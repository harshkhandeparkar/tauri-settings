import { getSettings } from './get-settings';
import { saveSettings } from './save-settings';

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
