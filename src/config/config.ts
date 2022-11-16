import { CONFIG_DEFAULTS } from '../constants';

export interface IConfig {
  /**
   * The name of the file in which the settings should be saved.
   * The filename should be without any extension.
   * Default: 'settings'
   */
  fileName: string;
  /**
   * Path to the directory in which the settings file is to be created.
   * Default: User's app config directory. See https://tauri.app/v1/api/js/path#appconfigdir
   */
  dir: string | null;
  /**
   * Whether or not to prettify the JSON settings data before storing to the file.
   * Default: false
   */
  prettify: boolean;
  /**
   * The number of spaces to use when prettifying the data. Will only work if the 'prettify' option is enabled.
   * Default: 2
   */
  numSpaces: number;
}

export type ConfigOptions = IConfig | {};

/**
 * @internal
 */
export function parseOptions(options: ConfigOptions): IConfig {
  const finalConfig: IConfig = {
    ...CONFIG_DEFAULTS,
    ...options
  }

  finalConfig.fileName = finalConfig.fileName.split('.')[0] + '.json';

  return finalConfig;
}
