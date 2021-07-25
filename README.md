## Tauri Settings
A user settings manager for [Tauri](https://tauri.studio) inspired by [electron-settings](https://github.com/nathanbuchar/electron-settings).

### Table of Contents
- [Installation And Usage](#installation-and-usage)
- [Differences From `electron-settings`](#differences-from-electron-settings)
- [API Docs]
- [License](LICENSE)

### Installation And Usage
#### Install The Package
The package is available on npm and can be installed using npm or yarn.
```shell
# using npm
npm install tauri-settings

# using yarn
yarn add tauri-settings
```

#### Enable Tauri APIs
The following APIs need to be added to the Tauri [allowlist](https://tauri.studio/en/docs/api/config#tauri.allowlist).
```json
{
  "allowlist": {
    "fs": { // see https://tauri.studio/en/docs/api/config#tauri.allowlist.fs
      "createDir": true,
      "readDir": true,
      "readTextFile": true,
      "writeFile": true
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

- `async has<SettingsSchema>(key)`: Async function that resolves with a boolean which is true if the given key exists in the settings.
- `async get<SettingsSchema>(key)`: Async function that resolves with the value of the setting corresponding to the given key.
- `async set<SettingsSchema>(key, value)`: Async function that sets the value of a given setting. Resolves with the entire settings object.
- `async getAll<SettingsSchema>()`: Async function that resolves with the entire settings object.


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

See the complete [API Docs]()

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

`SettingsManager` class can also be initialized with the `SettingsSchema` generic. (see [Usage](#usage))

#### Examples
```ts
import { SettingsManager } from 'tauri-settings';

type Schema = {
  theme: 'dark' | 'light';
  startFullscreen: boolean;
}

const settingsManager = new SettingsManager<Schema>(
  { // defaults
    theme: 'light',
    startFullscreen: false
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

See the complete [API Docs]()
