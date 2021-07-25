import { getSettings } from '../fs/get-settings';

/**
 * Checks whether a key exists in the settings.
 * @param key The key for the setting
 */
export async function has
  <SettingsSchema extends {} = any>
  (key: string): Promise<boolean>
{
  return (key in (await getSettings<SettingsSchema>()).settings);
}
