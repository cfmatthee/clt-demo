# Central Limit Theorem Demonstration

This application illustrates the Central Limit Theorem, specifically how it
tends to a Gaussian distribution when multiple probability distributions are
added. As such, distributions that are far away from Gaussian are available
to add to the existing distribution.

Although this is a tool that I use in a course, this is also a test bed for
different technologies. As such, the language and frameworks may change
for no reason other that I wanted too.

## Release versions

When implemented, GitHub actions are used to generate release versions.
A release version can be downloaded from the [Releases](releases) page. The
description will contain the technologies used in that release.

## Available versions

Tags in this repository indicate released versions. It can be build from
source after installing all the tools and dependencies.

### Version 3.4 - Tauri

The files in the `tauri` folder use [Rust](https://rust-lang.org/) with the
[Tauri](https://tauri.app/) framework. The Tauri tools will build the 
JavaScript ui using the [Svelte](https://svelte.dev/) framework (located in
the `ui` folder).

To compile locally, install the Rust and Node and the prerequisites for Tauri
(including the tauri cargo build tool). Install all the JavaScript
dependencies in the `ui` folder.
