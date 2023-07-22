import { invoke } from '@tauri-apps/api/tauri'
import { Path, PathValue } from './types/dot-notation';

export async function has<
  SettingsSchema,
  K extends Path<SettingsSchema> = Path<SettingsSchema>
> (key: K): Promise<boolean>
{
  return await invoke('plugin:settings|has', { key })
}

export async function get<
  SettingsSchema,
  K extends Path<SettingsSchema> = Path<SettingsSchema>
> (key: K): Promise<PathValue<SettingsSchema, K>>
{
  return await invoke('plugin:settings|get', { key })
}

export async function set<
  SettingsSchema,
  K extends Path<SettingsSchema> = Path<SettingsSchema>,
  V extends PathValue<SettingsSchema, K> = PathValue<SettingsSchema, K>
> (key: K, value: V): Promise<SettingsSchema>
{
  return await invoke('plugin:settings|set', { key, value })
}