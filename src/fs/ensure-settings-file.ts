import { appDir, resolvePath } from '@tauri-apps/api/path';
import { BaseDirectory, createDir, readDir, readTextFile, writeFile } from '@tauri-apps/api/fs';

import { ConfigOptions, parseOptions } from '../config/config';

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
export async function ensureSettingsFile(options: ConfigOptions = {}): Promise<{
  status: STATUS,
  path: string,
  content: string,
}> {
  const finalConfig = parseOptions(options);
  const settingsFilePath = await resolvePath(finalConfig.fileName, BaseDirectory.App);

  // create appDir()
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
