# Central Limit Theorem Demonstration

This application illustrates the Central Limit Theorem, specifically how it tends to a Guassian distribution when multiple probability distributions are added. As such, distributions that are far away from Gaussian are available to add to the existing distribution.

This application uses [Tauri](https://tauri.app/) with the frontend (GUI) build with [Astro](https://astro.build/).

## Building

First install the [prerequisites for Tauri](https://tauri.app/v1/guides/getting-started/prerequisites) and the [prerequisites for Astro](https://docs.astro.build/en/install/auto/#prerequisites). After cloning the repository, run `npm install` to install all the JavaScript dependencies.

To run a development version, run `cargo tauri dev`. To build the target for the local platform, run `cargo tauri build`. The resulting executable will be in the `src-tauri/target/release` folder.

This application was developed on Debian, but the final target will be used on Windows. To cross compile the application to Windows, the npm build target (`npm run build`) was adapted with the correct runner and target. The [list of prerequisites](https://tauri.app/v1/guides/building/cross-platform) is available on the Tauri web site. The resulting executable will be in the `src-tauri/target/x86_64-pc-windows-msvc/release` folder.

The generation of installers (bundles) are turned off. This setting can be changed in the `tauri.conf.json` file. Only the NSIS installer option is selected. This can also be changed in the same file.

## Licence

Copyright (c) 2024 Chris Matthee

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the "Software"), to deal in
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
of the Software, and to permit persons to whom the Software is furnished to do
so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
