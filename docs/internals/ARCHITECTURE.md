<!--
SPDX-FileCopyrightText: 2025 Jesse Aubin <jesseb34r@jesseaubin.me>

SPDX-License-Identifier: CC-BY-4.0
-->

# Architecture

## Project Structure

- `frontend` - SolidJS user interface.
- `crates/lotuslab-core` - Tauri app backend (app setup, wiring, commands).
- `crates/lotuslab-db` - Db repository implementations.
- `crates/lotuslab-services` - Application logic.
- `crates/lotuslab-types` - Application type and trait definitions.
- `crates/lotuslab-external` - Interface to mtgjson and potentially other sources.

## Technology Stack

- tauri
- surrealdb
- mtgjson
- rust
- solidjs
- tailwindcss
- kobalte (and solid-ui)
