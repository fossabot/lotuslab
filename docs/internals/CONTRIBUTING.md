<!--
SPDX-FileCopyrightText: 2025 Jesse Aubin <jesseb34r@jesseaubin.me>

SPDX-License-Identifier: CC-BY-4.0
-->

# Contributing to LotusLab

## Prerequisites

- compiler suite
    - linux: gcc or clang
    - macos: clang (xcode command line tools)
    - windows: msvc (install Visual Studio or the Visual Studio Build Tools)
        - make sure to install the "Desktop development with C++" workload
        - any Visual Studio edition will work (Community is free)
- rust toolchain
    - use rustup
    - use nightly release channel for best compiler performance
- bun

- TODO: have a more in depth guide for people with a completely fresh system

## Building

- TODO: this isn't tested, make sure there are no other build steps

```sh
git clone https://github.com/jesseb34r/lotuslab.git
cd lotuslab
cargo tauri build
```

## Docs

Developer-oriented documentation is at [/docs/internals/README.md](/docs/internals/README.md)

## Pull Requests

- TODO: figure out pull request contribution flow
- TODO: figure out a feature/fix/branching strategy
- TODO: figure out a issue tracking, roadmap, and todo strategy
