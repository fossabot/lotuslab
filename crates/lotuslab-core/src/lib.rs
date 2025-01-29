// SPDX-FileCopyrightText: 2025 Jesse Aubin <jesseb34r@jesseaubin.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::path::PathBuf;

use anyhow::Error;
use log::LevelFilter;
use lotuslab_db::{Store, open_store};
use specta_typescript::Typescript;
use tauri::{
    Manager,
    async_runtime::{Mutex, block_on},
    generate_context,
};
use tauri_specta::{Builder, collect_commands};

mod commands;
use commands::*;

pub(crate) struct DbState {
    store: Store,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = Builder::<tauri::Wry>::new().commands(collect_commands![
        get_folder_metadata,
        get_folder_children,
        new_folder,
        rename_folder,
        move_folder,
        delete_folder
    ]);

    #[cfg(debug_assertions)] // <- Only export on non-release builds
    builder
        .export(Typescript::default(), "../../frontend/src/bindings.ts")
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(LevelFilter::Info)
                .build(),
        )
        .invoke_handler(builder.invoke_handler())
        .setup(move |app| {
            let app_dir = app
                .path()
                .app_data_dir()
                .expect("failed to get app data directory");
            let store = block_on(setup_db(app_dir)).expect("failed to initialize database store");
            app.manage(Mutex::new(DbState { store }));

            Ok(())
        })
        .run(generate_context!())
        .expect("error while running tauri application");
}

async fn setup_db(app_data_dir: PathBuf) -> Result<Store, Error> {
    let db_path = app_data_dir.join("db");
    open_store(db_path).await.map_err(Error::msg)
}
