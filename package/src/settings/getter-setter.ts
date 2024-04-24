import type { Path, PathValue } from '../utils/dot-notation';
import { has as invokeHas, get as invokeGet, set as invokeSet } from '../utils/handlers';

/**
 * Checks whether a key exists in the settings.
 * @param key The key for the setting. Key supports dot notation. See https://github.com/harshkhandeparkar/tauri-settings#dot-notation.
 */
export async function has<
	SettingsSchema = {},
	K extends Path<SettingsSchema> = Path<SettingsSchema>
	>(key: K, fileId?: number): Promise<boolean> {
	try {
		return await invokeHas(key as string, fileId);
	}
	catch (e) {
		throw e;
	}
}


/**
 * Get the value of a particular setting.
 * @param key The key for the setting. Key supports dot notation. See https://github.com/harshkhandeparkar/tauri-settings#dot-notation.
 * @returns The value of the setting
 */
export async function get<
	SettingsSchema = {},
	K extends Path<SettingsSchema> = Path<SettingsSchema>
	>(key: K, fileId?: number): Promise<PathValue<SettingsSchema, K>> {
	try {
		return await invokeGet(key as string, fileId);
	}
	catch (e) {
		throw e;
	}
}

/**
 * Sets the value of a particular setting.
 * @param key The key for the setting. Key supports dot notation. See https://github.com/harshkhandeparkar/tauri-settings#dot-notation.
 * @param value The new value
 * @returns The entire settings object
 */
export async function set<
	SettingsSchema = {},
	K extends Path<SettingsSchema> = Path<SettingsSchema>,
	V extends PathValue<SettingsSchema, K> = PathValue<SettingsSchema, K>
	>(key: K, value: V, fileId?: number): Promise<void> {
	try {
		await invokeSet(key as string, value, fileId);
	}
	catch (e) {
		throw e;
	}
}
