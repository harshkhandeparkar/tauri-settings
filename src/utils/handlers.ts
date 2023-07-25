import { invoke } from '@tauri-apps/api';
import { IConfigOptions } from './config';

export async function has(key: string, customConfig?: IConfigOptions): Promise<boolean> {
	return await invoke('plugin:settings|has', { key, customConfig });
}

export async function get(key: string, customConfig?: IConfigOptions): Promise<any> {
	return await invoke('plugin:settings|get', { key, customConfig });
}

export async function set(key: string, value: any, customConfig?: IConfigOptions): Promise<any> {
	return await invoke('plugin:settings|set', { key, value, customConfig });
}

export async function overwrite_settings(new_settings: any, customConfig?: IConfigOptions): Promise<void> {
	await invoke('plugin:settings|overwrite_settings', { new_settings, customConfig });
}

export async function read_settings(customConfig?: IConfigOptions): Promise<[settings: any, settings_file_path: string, was_created: boolean]> {
	return await invoke('plugin:settings|read_settings', { customConfig });
}