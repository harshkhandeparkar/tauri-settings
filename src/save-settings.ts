import { writeFile } from '@tauri-apps/api/fs';

export async function saveSettings
  <SettingsSchema extends any>
  (newSettings: SettingsSchema, path: string)
{
  return await writeFile({
    contents: JSON.stringify(newSettings),
    path
  })
}
