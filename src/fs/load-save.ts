import { ensureSettingsFile, STATUS } from './ensure-settings-file';
import { writeFile } from '@tauri-apps/api/fs';

/**
 * @internal
 */
export async function saveSettings
  <SettingsSchema extends any>
  (newSettings: SettingsSchema, path: string)
{
  return await writeFile({
    contents: JSON.stringify(newSettings),
    path
  })
}


/**
 * Get all the settings.
 * @returns The entire settings object.
 */
export async function getSettings
  <SettingsSchema extends any>
  (): Promise<{settings: SettingsSchema, path: string, status: STATUS}>
{
  try {
    const settingsFile = await ensureSettingsFile();

    return {
      settings: JSON.parse(settingsFile.content) as SettingsSchema,
      path: settingsFile.path,
      status: settingsFile.status
    }
  }
  catch (e) {
    throw e;
  }
}
