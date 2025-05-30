import { ensureSettingsFile, STATUS } from "./ensure-settings-file";
import { writeFile } from "@tauri-apps/plugin-fs";
import { ConfigOptions, parseOptions } from "../config/config";

/**
 * @internal
 */
export async function saveSettings<SettingsSchema extends any>(
  newSettings: SettingsSchema,
  path: string,
  options: ConfigOptions,
) {
  try {
    const finalConfig = parseOptions(options);

    return await writeFile(
      path,
      JSON.stringify(
        newSettings,
        null,
        finalConfig.prettify ? finalConfig.numSpaces : 0,
      ),
    );
  } catch (e) {
    throw e;
  }
}

/**
 * Get all the settings.
 * @returns The entire settings object.
 */
export async function getSettings<SettingsSchema extends any>(
  options: ConfigOptions = {},
): Promise<{ settings: SettingsSchema; path: string; status: STATUS }> {
  try {
    const settingsFile = await ensureSettingsFile(options);

    return {
      settings: JSON.parse(settingsFile.content) as SettingsSchema,
      path: settingsFile.path,
      status: settingsFile.status,
    };
  } catch (e) {
    throw e;
  }
}
