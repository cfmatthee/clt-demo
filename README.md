# Central Limit Theorem Demonstration

This application illustrates the Central Limit Theorem, specifically how it tends to a Gaussian distribution when multiple probability distributions are added. As such, distributions that are far away from Gaussian are available to add to the existing distribution.

This application uses [Tauri](https://tauri.app/) with the frontend (GUI) build with [SolidStart](https://docs.solidjs.com/solid-start/).

## Building

First install the [prerequisites for Tauri](https://tauri.app/v1/guides/getting-started/prerequisites). After cloning the repository, run `npm install` in both the root and `ui` folders to install all the JavaScript dependencies.

To run a development version, first install the Tauri CLI application (either the Rust or JavaScript version). With the Tauri CLI installed, run either `cargo tauri dev` or `npm run tauri dev` or just `npm run dev`. To build the target for the local platform, run `cargo tauri build`, `npm run tauri build` or `npm run build`. The resulting executable will be in the `src-tauri/target/release` folder. The release version can be previewed using `npm run start`.

GitHub actions are used to generate release versions when a commit is pushed to the `release` branch.
