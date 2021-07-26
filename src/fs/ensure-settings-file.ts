import { appDir, resolvePath } from '@tauri-apps/api/path';
import { BaseDirectory, createDir, readDir, readTextFile, writeFile } from '@tauri-apps/api/fs';

import { SETTINGS_FILE } from '../constants';

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
export async function ensureSettingsFile(): Promise<{
  status: STATUS,
  path: string,
  content: string
}> {
  const settingsFilePath = await resolvePath(SETTINGS_FILE, BaseDirectory.App);

  try {
    await readDir(await appDir());
  }
  catch (e) {
    // doesn't exist
    try {
      await createDir(await appDir());
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
  catch(e) {
    // doesn't exist

      try {
      await writeFile({
        contents: JSON.stringify({}),
        path: settingsFilePath
      })

      return {
        status: STATUS.FILE_CREATED,
        path: settingsFilePath,
        content: JSON.stringify({})
      }
    }
    catch (e) {
      throw e;
    }
  }
}
