import { ensureSettingsFile, STATUS } from './ensure-settings-file';
import { writeFile } from '@tauri-apps/api/fs';
import { ConfigOptions, IConfig, parseOptions } from '../config/config';

/**
 * @internal
 */
export async function saveSettings
  <SettingsSchema extends any>
  (newSettings: SettingsSchema, path: string, config: IConfig)
{
  try {
    return await writeFile({
      contents: JSON.stringify(newSettings, null, config.prettify ? config.numSpaces : 0),
      path
    })
  }
  catch (e) {
    throw e;
  }
}


/**
 * Get all the settings.
 * @returns The entire settings object.
 */
export async function getSettings
  <SettingsSchema extends any>
  (config: IConfig): Promise<{ settings: SettingsSchema, path: string, status: STATUS }>
{
  try {
    const settingsFile = await ensureSettingsFile(config);

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
