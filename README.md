## Tauri Settings
A user settings manager for [Tauri](https://tauri.app) inspired by [electron-settings](https://github.com/nathanbuchar/electron-settings).

### Table of Contents
- [Installation And Usage](#installation-and-usage)
- [Differences From `electron-settings`](#differences-from-electron-settings)
- [API Docs](https://harshkhandeparkar.github.io/tauri-settings/)
- [License](LICENSE)

### Installation And Usage
#### Install The Package
The package is available on npm and can be installed using npm or yarn.
```shell
# using npm
npm install tauri-settings

# using yarn
yarn add tauri-settings

# using pnpm
pnpm add tauri-settings
```

#### Install The Tauri API
If you haven't installed `@tauri-apps/api` then you will have to install it using npm or yarn as this package internally uses the API.
```shell
# using npm
npm install @tauri-apps/api

# using yarn
yarn add @tauri-apps/api
```

#### Enable Tauri APIs
The following APIs need to be added to the Tauri [allowlist](https://tauri.app/v1/api/config/#allowlistconfig).
```jsonc
{
  "allowlist": {
    "fs": { // see https://tauri.app/v1/api/config/#fsallowlistconfig
      "createDir": true,
      "readDir": true,
      "readFile": true,
      "writeFile": true,
      "scope": ["$APPCONFIG", "$APPCONFIG/*"]
    },
    "path": {
      "all": true
     }
  }
}
```

#### Usage
`tauri-settings` exports a set of [standalone functions](#standalone-functions) for quick usage or a [`SettingsManager`](#settingsmanager) class with extra features such as caching.
Typescript typings and JSDoc is provided for all the API methods.

The API also uses typescript [generics](https://www.typescriptlang.org/docs/handbook/2/generics.html#hello-world-of-generics) to allow a defined schema to be used. In the following sections, `SettingsSchema` is an optional generic type for the settings schema.
It is highly recommended to use a defined schema to prevent runtime errors.

[`SettingsManager`](#settingsmanager) class can also be initialized with a `SettingsSchema` generic. This generic will be used by all the methods of the class instance.
Apart from basic setters and getters, the [`SettingsManager`](#settingsmanager) class also caches the value of the settings in the memory for quick access. This can also be used to make the api calls synchronous. See [Differences From `electron-settings`: Asynchronous](#asynchronous).

Using both the standalone methods and `SettingsManager` together can cause unexpected behaviour. If a setting is accessed both by the frontend and the backend then not using the caching feature is recommended.

#### Standalone Functions
`tauri-settings` exports the following API methods to directly set or get settings for quick usage. Alternatively you can also use [`SettingsManager`](#settingsmanager).
Each of the following methods has an `options` parameter. See the [Config](#config) to learn more.

- `async has<SettingsSchema>(key, options = {})`: Async function that resolves with a boolean which is true if the given key exists in the settings.
- `async get<SettingsSchema>(key, options = {})`: Async function that resolves with the value of the setting corresponding to the given key.
- `async set<SettingsSchema>(key, value, options = {})`: Async function that sets the value of a given setting. Resolves with the entire settings object.
- `async getAll<SettingsSchema>(, options = {})`: Async function that resolves with the entire settings object.

Here `key` uses [dot notation](#dot-notation).

#### Examples
```ts
type Schema = {
  theme: 'dark' | 'light';
  startFullscreen: boolean;
}

get<Schema>('theme').then((theme) => {
  // change the theme
})

// when the theme is changed by the user
set<Schema>('theme').then(() => console.log('theme changed succesfully'));
```

See the complete [API Docs](https://harshkhandeparkar.github.io/tauri-settings/).

#### SettingsManager
`SettingsManager` is a class that can be used not only to set and get settings but it is meant to be a complete settings *manager*.
It provides additional features such as caching the settings in the memory, setting defaults and in the future, listening to changes in the settings.

The caching feature stores a copy of the settings on the RAM and does not directly alter the settings file on persistent storage. This can be useful in multiple cases:
- Quick access without making filesystem changes
- Updating settings in batch
- Using the API syncrhonously (See [Differences From `electron-settings`: Asynchronous](#asynchronous))
- Deferring the filesystem events to a more convenient time

The cached settings can be accessed by using the `hasCache`, `getCache` or `setCache` methods.
The cache can be synced (written to persistent storage) at any time or the persistent storage can be accessed at any time using the `has`, `get` and `set` methods.

[Dot notation](#dot-notation) is also supported here.

`SettingsManager` class can also be initialized with the `SettingsSchema` generic. (see [Usage](#usage))

#### Examples
```ts
// TypeScript

import { SettingsManager } from 'tauri-settings';

type Schema = {
  theme: 'dark' | 'light';
  startFullscreen: boolean;
}

const settingsManager = new SettingsManager<Schema>(
  { // defaults
    theme: 'light',
    startFullscreen: false
  },
  { // options
    fileName: 'customization-settings'
  }
)

// checks whether the settings file exists and created it if not
// loads the settings if it exists
settingsManager.initialize().then(() => {
  // any key other than 'theme' and 'startFullscreen' will be invalid.
  // theme key will only accept 'dark' or 'light' as a value due to the generic.
  settingsManager.setCache('theme', 'dark');
}

// at a later time
await settingsManager.syncCache();
```

```js
// JavaScript

import { SettingsManager } from 'tauri-settings';

const settingsManager = new SettingsManager(
  { // defaults
    theme: 'light',
    startFullscreen: false
  },
  { // options
    fileName: 'customization-settings'
  }
);

// checks whether the settings file exists and created it if not
// loads the settings if it exists
settingsManager.initialize().then(() => {
  // there is no schema, so any key will be accepted
  // the user needs to add their own validation scheme
  settingsManager.setCache('theme', 'dark');
}

// at a later time
await settingsManager.syncCache();

```

See the complete [API Docs](https://harshkhandeparkar.github.io/tauri-settings/).

### Differences From `electron-settings`
#### Asynchronous
Since the Tauri [`fs` API](https://tauri.app/v1/api/js/fs) is asynchronous, the API methods exported by `tauri-settings` are also asynchronous. Methods `setSync`, `getSync`, and `hasSync` from `electron-settings` are not available.

Even though synchronous `fs` API is not available, the caching feature of [`SettingsManager`](#settingsmanager) can be used to synchronously set and read the settings.

#### Dot Notation
`electron-settings` allows you to access settings by using [dot notation](https://electron-settings.js.org/index.html#keypath).
`tauri-settings` supports (Thanks to https://github.com/harshkhandeparkar/tauri-settings/pull/3) the above feature without the array notation `key.array[4]`.

Example:
If the settings schema looks like this:
```js
{
  theme: {
    mode: 'dark',
    accent: 'red'
  }
}
```
`get('theme.mode')` will return `dark`.

The following will NOT work:
```js
{
  search: {
    recents: ['keyword1', 'keyword2', 'keyword3']
  }
}
```
`get('search.recents[3]')` will return `null` whereas `get('search.recents')` will return the entire `recents` array.

#### Config
`electron-settings` exports a [`configure()`](https://electron-settings.js.org/index.html#configure) method to configure some of the options such as the fileName.
However, `tauri-settings` doesn't export such a variable due to various reasons. Instead each API method such as `get` and `set`, as well as the `SettingsManager` class have an optional `options` parameter (See [API Docs](https://harshkhandeparkar.github.io/tauri-settings/)).

****
#### Thank You
