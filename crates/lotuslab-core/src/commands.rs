// SPDX-FileCopyrightText: 2025 Jesse Aubin <jesseb34r@jesseaubin.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use specta::specta;
use tauri::{State, async_runtime::Mutex, command};

use lotuslab_types::{Folder, FolderChildren, FolderId, NewFolder};

use crate::DbState;

// ----- Folder Commands -----

#[command]
#[specta]
pub(crate) async fn get_folder_metadata(
    state: State<'_, Mutex<DbState>>,
    id: FolderId,
) -> Result<Folder, String> {
    let store = &state.lock().await.store;
    lotuslab_services::get_folder_metadata(store, &id).await
}

#[command]
#[specta]
pub(crate) async fn get_folder_children(
    state: State<'_, Mutex<DbState>>,
    id: FolderId,
) -> Result<FolderChildren, String> {
    let store = &state.lock().await.store;
    lotuslab_services::get_folder_children(store, &id).await
}

#[command]
#[specta]
pub(crate) async fn new_folder(
    state: State<'_, Mutex<DbState>>,
    new_folder: NewFolder,
) -> Result<Folder, String> {
    let store = &state.lock().await.store;
    lotuslab_services::new_folder(store, new_folder).await
}

#[command]
#[specta]
pub(crate) async fn rename_folder(
    state: State<'_, Mutex<DbState>>,
    id: FolderId,
    name: String,
) -> Result<Folder, String> {
    let store = &state.lock().await.store;
    lotuslab_services::rename_folder(store, &id, name).await
}

#[command]
#[specta]
pub(crate) async fn move_folder(
    state: State<'_, Mutex<DbState>>,
    id: FolderId,
    target_id: FolderId,
) -> Result<Folder, String> {
    let store = &state.lock().await.store;
    lotuslab_services::move_folder(store, &id, &target_id).await
}

#[command]
#[specta]
pub(crate) async fn delete_folder(
    state: State<'_, Mutex<DbState>>,
    id: FolderId,
) -> Result<(), String> {
    let store = &state.lock().await.store;
    lotuslab_services::delete_folder(store, &id).await
}
