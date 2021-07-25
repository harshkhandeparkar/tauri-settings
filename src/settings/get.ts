import { getSettings } from '../fs/get-settings';
import { has } from './has';

/**
 * Get the value of a particular setting.
 * @param key The key for the setting
 * @returns The value
 */
export async function get
  <SettingsSchema extends {} = any, K  extends keyof SettingsSchema = keyof SettingsSchema>
  (key: K): Promise<SettingsSchema[K]>
{
  if (await has<SettingsSchema>(key as string)) return (await getSettings<SettingsSchema>()).settings[key];
  else throw 'Error: key does not exist.'
}
