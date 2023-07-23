import { appConfigDir, join } from '@tauri-apps/api/path';
import { createDir, readDir, readTextFile, writeFile } from '@tauri-apps/api/fs';

import { IConfig, parseOptions } from '../config/config';

/**
 * @internal
 */
export enum STATUS {
	FILE_EXISTS = 'file_exists',
	FILE_CREATED = 'file_created'
}

/**
 * @internal
 */
export async function ensureSettingsFile(config: IConfig): Promise<{
	status: STATUS,
	path: string,
	content: string,
}> {
	try {
		const finalConfig = parseOptions(config);
		const finalDir = finalConfig.dir ?? await appConfigDir();

		const settingsFilePath = await join(finalDir, finalConfig.fileName);

		// create appConfigDir()
		try {
			await readDir(finalDir);
		}
		catch (e) {
			// doesn't exist
			try {
				await createDir(finalDir, { recursive: true });
			}
			catch (e) {
				throw e;
			}
		}

		try {
			const content = await readTextFile(settingsFilePath);

			return {
				status: STATUS.FILE_EXISTS,
				path: settingsFilePath,
				content
			}
		}
		catch (e) {
			// doesn't exist

			try {
				await writeFile({
					contents: JSON.stringify({}, null, finalConfig.prettify ? finalConfig.numSpaces : 0),
					path: settingsFilePath
				})

				return {
					status: STATUS.FILE_CREATED,
					path: settingsFilePath,
					content: JSON.stringify({}, null, finalConfig.prettify ? finalConfig.numSpaces : 0)
				}
			}
			catch (e) {
				throw e;
			}
		}
	}
	catch (e) {
		throw e;
	}
}
