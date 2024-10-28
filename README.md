# Central Limit Theorem Demonstration

This application illustrates the Central Limit Theorem, specifically how it
tends to a Gaussian distribution when multiple probability distributions are
added. As such, distributions that are far away from Gaussian are available
to add to the existing distribution.

Although this is a tool that I use in a course, this is also a test bed for
different technologies. As such, the language and frameworks may change
for no reason other that I wanted too.

## Building

This application uses [Tauri](https://tauri.app/) with the frontend (GUI) build
with [SolidStart](https://docs.solidjs.com/solid-start/).

First install the
[prerequisites for Tauri](https://tauri.app/v1/guides/getting-started/prerequisites).
After cloning the repository, run `npm install` in the root folder to install
all the JavaScript dependencies.

To run a development version, first install the Tauri CLI application (either
the Rust or JavaScript version). With the Tauri CLI installed, run
`npm run dev`. To build the target for the local platform, run `npm run build`.
The resulting executable will be in the `src-tauri/target/release` folder. The
release version can be previewed using `npm run start`.

GitHub actions are used to generate release versions.
