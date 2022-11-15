import type { Path, PathValue } from '../types/dot-notation';

/**
 * @internal
 */
export function getDotNotation<SettingsSchema, K extends Path<SettingsSchema>>(obj: SettingsSchema, path: K): PathValue<SettingsSchema, K> | undefined {
    if (typeof path !== 'string') throw 'Error: path must be a string';

    const keys = path.split('.');

    let current: any = obj;
    for (let i = 0; i < keys.length; i++) {
        const key = keys[i];

        if (current[key] === undefined) {
            return undefined;
        } else {
            current = current[key];
        }
    }

    return current;
}

/**
 * @internal
 */
export function setDotNotation<SettingsSchema, K extends Path<SettingsSchema>>(obj: SettingsSchema, path: K, value: PathValue<SettingsSchema, K>): PathValue<SettingsSchema, K> | undefined {
    if (typeof path !== 'string') throw 'Error: path must be a string';

    const keys = path.split('.');

    let current = obj;
    for (let i = 0; i < keys.length - 1; i++) {
        const key = keys[i];

        if (current[key] === undefined) {
            return undefined;
        }

        current = current[key];
    }

    if (current[keys[keys.length - 1]] === undefined) return undefined;

    current[keys[keys.length - 1]] = value;
    return value;
}