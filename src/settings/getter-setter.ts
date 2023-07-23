import { ConfigOptions, parseOptions } from '../config/config';
import { getSettings, saveSettings } from '../fs/load-save';
import { getDotNotation, setDotNotation } from '../utils/dot-notation';
import type { Path, PathValue } from '../types/dot-notation';
import { has as invokeHas, get as invokeGet, set as invokeSet } from '../plugin/handlers';

/**
 * Checks whether a key exists in the settings.
 * @param key The key for the setting. Key supports dot notation. See https://github.com/harshkhandeparkar/tauri-settings#dot-notation.
 */
export async function has<
	SettingsSchema,
	K extends Path<SettingsSchema> = Path<SettingsSchema>
	>(key: K, options: ConfigOptions = {}): Promise<boolean> {
	const config = parseOptions(options);

	try {
		if (config.usePlugin) {
			return await invokeHas(key as string);
		}
		else {
			const { settings } = await getSettings<SettingsSchema>(config);
			const value = getDotNotation(settings, key);

			return value !== null;
		}
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
	SettingsSchema,
	K extends Path<SettingsSchema> = Path<SettingsSchema>
	>(key: K, options: ConfigOptions = {}): Promise<PathValue<SettingsSchema, K>> {
	const config = parseOptions(options);

	try {
		if (config.usePlugin) {
			return await invokeGet(key as string);
		}
		else {
			const { settings } = await getSettings<SettingsSchema>(config);
			return getDotNotation<SettingsSchema, K>(settings, key);
		}
	}
	catch (e) {
		throw e;
	}
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
	>(key: K, value: V, options: ConfigOptions = {}): Promise<SettingsSchema> {
	const config = parseOptions(options);

	try {
		if (config.usePlugin) {
			return await invokeSet(key as string, value);
		}
		else {
			const settings = await getSettings<SettingsSchema>(config);
			setDotNotation<SettingsSchema, K>(settings.settings, key, value);

			await saveSettings<SettingsSchema>(settings.settings, settings.path, config);

			return settings.settings;
		}
	}
	catch (e) {
		throw e;
	}
}
