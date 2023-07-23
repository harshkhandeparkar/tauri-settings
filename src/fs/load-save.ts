import { ensureSettingsFile, STATUS } from './ensure-settings-file';
import { writeFile } from '@tauri-apps/api/fs';
import { IConfig } from '../config/config';
import { overwrite_settings, read_settings } from '../plugin/handlers';
import { invoke } from '@tauri-apps/api';

/**
 * @internal
 */
export async function saveSettings
	<SettingsSchema extends any>
	(newSettings: SettingsSchema, path: string, config: IConfig): Promise<void> {
	try {
		if (config.usePlugin) {
			await overwrite_settings(newSettings);
		}
		else {
			await writeFile({
				contents: JSON.stringify(newSettings, null, config.prettify ? config.numSpaces : 0),
				path
			})
		}
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
	(config: IConfig): Promise<{ settings: SettingsSchema, path: string, status: STATUS }> {
	try {
		if (config.usePlugin) {
			let [settings, path, was_created] = await read_settings();

			return {
				settings,
				path: path,
				status: was_created ? STATUS.FILE_CREATED : STATUS.FILE_EXISTS,
			}
		}
		else {
			const settingsFile = await ensureSettingsFile(config);

			return {
				settings: JSON.parse(settingsFile.content) as SettingsSchema,
				path: settingsFile.path,
				status: settingsFile.status
			}
		}
	}
	catch (e) {
		throw e;
	}
}
