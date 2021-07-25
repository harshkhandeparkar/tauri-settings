import { getSettings } from './get-settings';

export async function has
  <SettingsSchema extends {} = any>
  (key: string): Promise<boolean>
{
  return (key in (await getSettings<SettingsSchema>()).settings);
}
