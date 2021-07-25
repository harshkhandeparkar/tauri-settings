import { ensureSettingsFile, STATUS } from './ensure-settings-file';

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
