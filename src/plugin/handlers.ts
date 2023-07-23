import { invoke } from '@tauri-apps/api';

export async function has(key: string): Promise<boolean> {
	return await invoke('plugin:settings|has', { key });
}

export async function get(key: string): Promise<any> {
	return await invoke('plugin:settings|get', { key });
}

export async function set(key: string, value: any): Promise<any> {
	return await invoke('plugin:settings|set', { key, value });
}

export async function overwrite_settings(new_settings: any): Promise<void> {
	await invoke('plugin:settings|overwrite_settings', { new_settings });
}

export async function read_settings(): Promise<[settings: any, settings_file_path: string, was_created: boolean]> {
	return await invoke('plugin:settings|read_settings');
}