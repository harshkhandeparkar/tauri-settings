import { IConfig } from '../config/config';
import { overwrite_settings, read_settings } from '../plugin/handlers';

/**
 * @internal
 */
export async function saveSettings
	<SettingsSchema extends any>
	(newSettings: SettingsSchema, config: IConfig): Promise<void> {
	try {
		await overwrite_settings(newSettings);
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
	(config: IConfig): Promise<{ settings: SettingsSchema, path: string, was_created: boolean }> {
	try {
		let [settings, path, was_created] = await read_settings();

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
