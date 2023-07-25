import { IConfigOptions } from '../utils/config';
import { overwrite_settings, read_settings } from '../utils/handlers';

/**
 * @internal
 */
export async function saveSettings
	<SettingsSchema extends any>
	(newSettings: SettingsSchema, customConfig?: IConfigOptions): Promise<void> {
	try {
		await overwrite_settings(newSettings, customConfig);
	}
	catch (e) {
		throw e;
	}
}


/**
 * Get all the settings.
 * @returns The entire settings object.
 */
export async function getSettings
	<SettingsSchema extends any>
	(customConfig?: IConfigOptions): Promise<{ settings: SettingsSchema, path: string, was_created: boolean }> {
	try {
		let [settings, path, was_created] = await read_settings(customConfig);

		return {
			settings,
			path: path,
			was_created,
		}
	}
	catch (e) {
		throw e;
	}
}
