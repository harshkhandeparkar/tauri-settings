import type { Path, PathValue } from '../utils/dot-notation';
import { has as invokeHas, get as invokeGet, set as invokeSet, get_settings_file_id } from '../utils/handlers';

/**
 * Checks whether a key exists in the settings.
 */
export async function has<
	SettingsSchema = {},
	K extends Path<SettingsSchema> = Path<SettingsSchema>
	>(
		/** The key for the setting. Key supports dot notation. See https://github.com/harshkhandeparkar/tauri-settings#dot-notation. */
		key: K,
		/** The settings file to search. Can search using the ID of the settings file or by a path (string) relative to the scope directory. */
		file?: number | string
	): Promise<boolean> {
	try {
		const fileId = typeof file === 'string' ? await get_settings_file_id(file) : file;

		if (fileId === null) {
			throw new Error(`Settings file \`${file}\` not found.`);
		}

		return await invokeHas(key as string, fileId);
	}
	catch (e) {
		throw e;
	}
}


/**
 * Get the value of a particular setting.
 */
export async function get<
	SettingsSchema = {},
	K extends Path<SettingsSchema> = Path<SettingsSchema>
	>(
		/** The key for the setting. Key supports dot notation. See https://github.com/harshkhandeparkar/tauri-settings#dot-notation. */
		key: K,
		/** The settings file to search. Can search using the ID of the settings file or by a path (string) relative to the scope directory. */
		file?: number | string
	): Promise<PathValue<SettingsSchema, K>> {
	try {
		const fileId = typeof file === 'string' ? await get_settings_file_id(file) : file;

		if (fileId === null) {
			throw new Error(`Settings file \`${file}\` not found.`);
		}

		return await invokeGet(key as string, fileId);
	}
	catch (e) {
		throw e;
	}
}

/**
 * Sets the value of a particular setting.
 */
export async function set<
	SettingsSchema = {},
	K extends Path<SettingsSchema> = Path<SettingsSchema>,
	V extends PathValue<SettingsSchema, K> = PathValue<SettingsSchema, K>
	>(
		/** The key for the setting. Key supports dot notation. See https://github.com/harshkhandeparkar/tauri-settings#dot-notation. */
		key: K,
		/** The new value to set. */
		value: V,
		/** The settings file to search. Can search using the ID of the settings file or by a path (string) relative to the scope directory. */
		file?: number | string
	): Promise<void> {
	try {
		const fileId = typeof file === 'string' ? await get_settings_file_id(file) : file;

		if (fileId === null) {
			throw new Error(`Settings file \`${file}\` not found.`);
		}

		await invokeSet(key as string, value, fileId);
	}
	catch (e) {
		throw e;
	}
}
