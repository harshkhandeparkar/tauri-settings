import { getSettings } from './get-settings';

export async function get
  <SettingsSchema extends any, K extends keyof SettingsSchema>
  (key: K): Promise<SettingsSchema[K]>
{
  return (await getSettings<SettingsSchema>()).settings[key];
}
