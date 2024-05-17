# Central Limit Theorem Demonstration

This application illustrates the Central Limit Theorem, specifically how it tends to a Gaussian distribution when multiple probability distributions are added. As such, distributions that are far away from Gaussian are available to add to the existing distribution.

This application uses [Tauri](https://tauri.app/) with the frontend (GUI) build with [Astro](https://astro.build/).

## Building

First install the [prerequisites for Tauri](https://tauri.app/v1/guides/getting-started/prerequisites) and the [prerequisites for Astro](https://docs.astro.build/en/install/auto/#prerequisites). After cloning the repository, run `npm install` to install all the JavaScript dependencies.

To run a development version, first install the Tauri CLI application (either the Rust or JavaScript version). With the Tauri CLI installed, run either `cargo tauri dev` or `npm run tauri dev` or just `npm run dev`. To build the target for the local platform, run `cargo tauri build` or `npm run tauri build`. The resulting executable will be in the `src-tauri/target/release` folder.

This application was developed on Debian, but the final target will be used on Windows. To cross compile the application to Windows, the npm build target (`npm run build`) was adapted with the correct runner and target. The [list of prerequisites](https://tauri.app/v1/guides/building/cross-platform) is available on the Tauri web site. The resulting executable will be in the `src-tauri/target/x86_64-pc-windows-msvc/release` folder.

The generation of installers (bundles) are turned off. This setting can be changed in the `tauri.conf.json` file. Only the NSIS installer option is selected. This can also be changed in the same file.
