import { invoke } from '@tauri-apps/api';
import { ISettingsFileOptions } from './settings_file';

export async function add_settings_file(settingsFileOptions: ISettingsFileOptions): Promise<number> {
	return await invoke('plugin:settings|add_settings_file', { settingsFileOptions });
}

export async function has(key: string, fileId?: number): Promise<boolean> {
	return await invoke('plugin:settings|has', { key, fileId });
}

export async function get(key: string, fileId?: number): Promise<any> {
	return await invoke('plugin:settings|get', { key, fileId });
}

export async function set(key: string, value: any, fileId?: number): Promise<void> {
	return await invoke('plugin:settings|set', { key, value, fileId });
}