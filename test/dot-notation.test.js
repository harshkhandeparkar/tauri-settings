const { getDotNotation, setDotNotation } = require('../src/utils/dot-notation');
const cloneDeep = require('lodash/cloneDeep');

const settings = {
	theme: {mode: 'dark', accent: 'red'}
}

test('getDotNotation() returns the right value', () => {
	expect(
		getDotNotation(
			cloneDeep(settings),
			'theme.mode'
		)
	).toBe('dark')
})

test('getDotNotation() handles errors correctly', () => {
	expect(() =>
		getDotNotation(
			cloneDeep(settings),
			4
		)
	).toThrow('Error: path must be a string')
})

test('getDotNotation() returns null for non-existent path', () => {
	expect(
		getDotNotation(
			cloneDeep(settings),
			'theme.mode.accent'
		)
	).toBeNull()
})

test('setDotNotation() sets the right value', () => {
	expect(
		setDotNotation(
			cloneDeep(settings),
			'theme.mode',
			'light'
		)
	).toStrictEqual({
		...settings,
		theme: {
			...settings.theme,
			mode: 'light'
		}
	})
})

test('setDotNotation() creates non-existent keys', () => {
	expect(
		setDotNotation(
			cloneDeep(settings),
			'theme.highContrast',
			true
		)
	).toStrictEqual({
		...settings,
		theme: {
			...settings.theme,
			highContrast: true
		}
	})

	expect(
		setDotNotation(
			cloneDeep(settings),
			'theme.font',
			{
				size: 16,
				bold: true
			}
		)
	).toStrictEqual({
		...settings,
		theme: {
			...settings.theme,
			font: {
				size: 16,
				bold: true
			}
		}
	})

	expect(
		setDotNotation(
			cloneDeep(settings),
			'theme.font.size',
			16
		)
	).toStrictEqual({
		...settings,
		theme: {
			...settings.theme,
			font: {
				size: 16
			}
		}
	})
})

test('setDotNotation() handles errors correctly', () => {
	expect(() =>
		setDotNotation(
			cloneDeep(settings),
			4,
			'string'
		)
	).toThrow('Error: path must be a string')
})