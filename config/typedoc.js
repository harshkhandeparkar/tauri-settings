const { readdirSync, statSync } = require('fs');
const { join } = require('path');

const getListOfFiles = (dirPath) => {
	const list = [];

	readdirSync(dirPath).map((name) => {
		if (statSync(join(dirPath, name)).isDirectory()) {
			return list.push(...getListOfFiles(join(dirPath, name)));
		}
		else list.push(join(dirPath, name));
	})

	return list;
}

module.exports = {
	excludeProtected: true,
	excludePrivate: true,
	excludeInternal: true,
	includeVersion: true,
	name: 'Tauri Settings',
	// entryPoints: getListOfFiles(join(__dirname, '..', 'src')),
	entryPoints: [join(__dirname, '..', 'src', 'index.ts')],
	out: [join(__dirname, '..', 'docs')],
	readme: 'none',
	theme: 'default'
}
