import { ensureSettingsFile } from './ensure-settings-file';

export async function getSettings
  <SettingsSchema extends any>
  (): Promise<{settings: SettingsSchema, path: string}>
{
  try {
    const settingsFile = await ensureSettingsFile();

    return {
      settings: JSON.parse(settingsFile.content) as SettingsSchema,
      path: settingsFile.path
    }
  }
  catch (e) {
    throw e;
  }
}
