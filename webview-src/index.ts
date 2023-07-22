import { invoke } from '@tauri-apps/api/tauri'
import { Path, PathValue } from './types/dot-notation';

/**
 * Checks whether a key exists in the settings.
 * @param key The key for the setting. Key supports dot notation. See https://github.com/harshkhandeparkar/tauri-settings#dot-notation.
 */
export async function has<
  SettingsSchema,
  K extends Path<SettingsSchema> = Path<SettingsSchema>
> (key: K): Promise<boolean>
{
  return await invoke('plugin:settings|has', { key })
}

/**
 * Get the value of a particular setting.
 * @param key The key for the setting. Key supports dot notation. See https://github.com/harshkhandeparkar/tauri-settings#dot-notation.
 * @returns The value of the setting
 */
export async function get<
  SettingsSchema,
  K extends Path<SettingsSchema> = Path<SettingsSchema>
> (key: K): Promise<PathValue<SettingsSchema, K>>
{
  return await invoke('plugin:settings|get', { key })
}


/**
 * Sets the value of a particular setting
 * @param key The key for the setting. Key supports dot notation. See https://github.com/harshkhandeparkar/tauri-settings#dot-notation.
 * @param value The new value
 * @returns The entire settings object
 */
export async function set<
  SettingsSchema,
  K extends Path<SettingsSchema> = Path<SettingsSchema>,
  V extends PathValue<SettingsSchema, K> = PathValue<SettingsSchema, K>
> (key: K, value: V): Promise<SettingsSchema>
{
  return await invoke('plugin:settings|set', { key, value })
}