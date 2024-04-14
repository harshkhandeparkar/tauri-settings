<div id="top"></div>

<!-- PROJECT SHIELDS -->
<div align="center">

[![Contributors][contributors-shield]][contributors-url]
[![Build Workflow][build-workflow-shield]][build-workflow-url]
[![Docs Workflow][docs-workflow-shield]][docs-workflow-url]
[![NPM Version][npm-shield]][npm-url]
[![MIT License][license-shield]][license-url]

</div>

<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/harshkhandeparkar/tauri-settings">
    <img width="140" alt="image" src="./img/tauri-settings-logo-circular.png">
  </a>

  <h3 align="center">Tauri Settings</h3>

  <p align="center">
    <p>A user settings manager library for Tauri inspired by electron-settings.</p>
    <a href="https://www.npmjs.com/package/tauri-settings">NPM</a>
    .
    <a href="https://harshkhandeparkar.github.io/tauri-settings/js">Javascript API Docs</a>
    ·
    <a href="https://harshkhandeparkar.github.io/tauri-settings/rust/tauri_plugin_settings">Rust API Docs</a>
  </p>
</div>


<!-- TABLE OF CONTENTS -->
## Table of Contents
- [About The Project](#about-the-project)
- [Getting Started](#getting-started)
  - [Installation](#installation)
    - [Migrating from v0.x.x](#migrating-from-v0xx)
  - [Usage](#usage)
    - [Dot Notation](#dot-notation)
    - [Differences from `electron-settings`](#differences-from-electron-settings)
    - [Examples](#examples)
- [Acknowledgements](#acknowledgments)

<!-- ABOUT THE PROJECT -->
## About The Project
`tauri-settings` is a user settings manager library for [Tauri](https://tauri.app) inspired by [electron-settings](https://github.com/nathanbuchar/electron-settings).

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- GETTING STARTED -->
## Getting Started

### Installation
1. Install the `tauri-plugin-settings` Tauri plugin.
  - Run the command `cargo add tauri-plugin-settings` inside `src-tauri`. This will add the package to the `Cargo.toml` file.
  - Load the plugin in the `src-tauri/main.rs` file. See the [plugin docs](https://harshkhandeparkar.github.io/tauri-settings/rust/tauri_plugin_settings#getting-started) for loading instructions.
2. Install the `tauri-settings` JS plugin wrapper.
  - Using npm: `npm install tauri-settings`.
  - Using pnpm: `pnpm add tauri-settings`.
  - Using yarn: `yarn add tauri-settings`.
  - See the [Usage](#usage) section for usage instructions.

#### Migrating from v0.x.x
`v0.x.x` of Tauri Settings was a 100% Javascript library that used the [Tauri API](https://tauri.app/v1/api/js/) to read and write settings to the disk. `v1.x.x` is written as a Rust [Tauri plugin](https://tauri.app/v1/guides/features/plugin) with a JS wrapper for the frontend.

To migrate from `v0.x.x` to `v1.x.x`, the Tauri plugin has to be installed and enabled (follow the [installation steps](#installation)). The frontend API remains the same with the only change being `hasCache`, `getCache`, and `setCache` being converted into `async` functions as the caching is now handled on the Rust side.

> ![NOTE] The caching may change in the future to support JS-side caching for faster performance, but it will likely be a config option and use the same async API.

> ![WARNING] Handling all filesystem operations on Rust-side doesn't automatically secure all data. This just prevents JS-side from having access to the filesystem APIs, and only allows settings read/write operations. The data is still written in plaintext to JSON files, so it is not recommended to use this for storing any secrets. Other plugins such as [stronghold](https://github.com/tauri-apps/tauri-plugin-stronghold) are more suitable.

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- USAGE EXAMPLES -->
### Usage


#### Dot Notation
#### Differences from `electron-settings`

#### Examples

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- ACKNOWLEDGMENTS -->
## Acknowledgments
* [Choose an Open Source License](https://choosealicense.com)
* [Img Shields](https://shields.io)
* [@proffapt's README](https://github.com/proffapt/myREADME/)
* [Canva](https://canva.com) (Logo design)

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- MARKDOWN LINKS & IMAGES -->

[contributors-shield]: https://img.shields.io/github/contributors/harshkhandeparkar/tauri-settings.svg?style=for-the-badge
[contributors-url]: https://github.com/harshkhandeparkar/tauri-settings/graphs/contributors
[license-shield]: https://img.shields.io/github/license/harshkhandeparkar/tauri-settings.svg?style=for-the-badge
[license-url]: https://github.com/harshkhandeparkar/tauri-settings/blob/master/LICENSE.txt
[build-workflow-shield]: https://img.shields.io/github/actions/workflow/status/harshkhandeparkar/tauri-settings/test_and_lint.yml?style=for-the-badge
[build-workflow-url]: https://github.com/harshkhandeparkar/tauri-settings/actions/workflows/test_and_lint.yml
[docs-workflow-shield]: https://img.shields.io/github/actions/workflow/status/harshkhandeparkar/tauri-settings/docs.yml?style=for-the-badge&label=Docs&color=blue
[docs-workflow-url]: https://harshkhandeparkar.github.io/tauri-settings
[npm-shield]: https://img.shields.io/npm/v/tauri-settings?style=for-the-badge
[npm-url]: https://www.npmjs.com/package/tauri-settings