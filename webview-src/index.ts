import { invoke } from '@tauri-apps/api/tauri'

export async function has(
  key: string | number | symbol
): Promise<boolean> {
  return await invoke('plugin:settings|has', { key })
}

export async function get<
  SettingsSchema extends {} = any,
  K extends keyof SettingsSchema = keyof SettingsSchema
>(key: K): Promise<SettingsSchema[K]> {
  return await invoke('plugin:settings|get', { key })
}

export async function set<
  SettingsSchema extends {} = any,
  K extends keyof SettingsSchema = keyof SettingsSchema
>(key: K, value: SettingsSchema[K]): Promise<SettingsSchema[K]> {
  return await invoke('plugin:settings|set', { key, value })
}