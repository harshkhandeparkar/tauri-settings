import type { Path, PathValue } from '../types/dot-notation';

/**
 * @internal
 */
export function getDotNotation<SettingsSchema, K extends Path<SettingsSchema> = Path<SettingsSchema>>(
	settings: SettingsSchema,
	path: K
): PathValue<SettingsSchema, K> | null {
	if (typeof path !== 'string') throw 'Error: path must be a string';

	const keys = path.split('.');

	let current: PathValue<SettingsSchema, K> | SettingsSchema = settings;
	for (let i = 0; i < keys.length; i++) {
		const key = keys[i];

		if (current[key] === undefined) {
			return null;
		} else {
			current = current[key];
		}
	}

	return current as PathValue<SettingsSchema, K>;
}

/**
 * @internal
 */
export function setDotNotation<SettingsSchema, K extends Path<SettingsSchema> = Path<SettingsSchema>>(
	settings: SettingsSchema,
	path: K,
	value: PathValue<SettingsSchema, K>
): SettingsSchema {
	if (typeof path !== 'string') throw 'Error: path must be a string';

	const keys = path.split('.');

	let current: PathValue<SettingsSchema, K> | SettingsSchema = settings;
	for (let i = 0; i < keys.length - 1; i++) {
		const key = keys[i];

		if (current[key] === undefined) {
			// if a key does not exist, create it
			current[key] = {};
		}

		current = current[key];
	}

	current[keys[keys.length - 1]] = value;
	return settings;
}