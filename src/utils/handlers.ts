import { invoke } from '@tauri-apps/api';
import { IConfigOptions } from './config';

export async function add_config(
	config: IConfigOptions,
	defaultSettings: any
): Promise<[config_id: number, settings: any]> {
	return await invoke('plugin:settings|add_config', { config, defaultSettings });
}

export async function has(key: string, configId?: number): Promise<boolean> {
	return await invoke('plugin:settings|has', { key, configId });
}

export async function has_cache(key: string, configId?: number): Promise<boolean> {
	return await invoke('plugin:settings|has_cache', { key, configId });
}

export async function get(key: string, configId?: number): Promise<any> {
	return await invoke('plugin:settings|get_cache', { key, configId });
}

export async function get_cache(key: string, configId?: number): Promise<any> {
	return await invoke('plugin:settings|get', { key, configId });
}

export async function set(key: string, value: any, configId?: number): Promise<any> {
	return await invoke('plugin:settings|set', { key, value, configId });
}

export async function set_cache(key: string, value: any, configId?: number): Promise<any> {
	return await invoke('plugin:settings|set_cache', { key, value, configId });
}

export async function cache_to_file(configId?: number): Promise<any> {
	return await invoke('plugin:settings|cache_to_file', { configId });
}

export async function file_to_cache(configId?: number): Promise<any> {
	return await invoke('plugin:settings|file_to_cache', { configId });
}