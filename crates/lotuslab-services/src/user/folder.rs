// SPDX-FileCopyrightText: 2025 Jesse Aubin <jesseb34r@jesseaubin.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use lotuslab_db::Store;
use lotuslab_types::{
    Folder, FolderChildren, FolderId, FolderReadRepo, FolderWriteRepo, NewFolder, Patch,
    PatchFolder, RepoError,
};

pub async fn get_folder_metadata(store: &Store, id: &FolderId) -> Result<Folder, String> {
    store
        .get_folder_metadata(id)
        .await
        .map_err(|e| e.to_string())
}

pub async fn get_folder_children(store: &Store, id: &FolderId) -> Result<FolderChildren, String> {
    store
        .get_folder_children(id)
        .await
        .map_err(|e| e.to_string())
}

pub async fn new_folder(store: &Store, new: NewFolder) -> Result<Folder, String> {
    store.create_folder(new).await.map_err(|e| e.to_string())
}

pub async fn rename_folder(store: &Store, id: &FolderId, name: String) -> Result<Folder, String> {
    let folder = store
        .get_folder_metadata(id)
        .await
        .map_err(|e| e.to_string())?;

    // noop
    if folder.name == name {
        return Ok(folder);
    };

    assert!(folder.parent.is_some());
    let peers = store
        .get_folder_children(&folder.parent.as_ref().unwrap())
        .await
        .map_err(|e| e.to_string())?;
    if peers.folders.iter().any(|peer| peer.name == name) {
        return Err("A folder with the same name already exists in this parent folder".to_string());
    }

    let renamed = store
        .update_folder(
            id,
            PatchFolder {
                name: Patch::Set(name),
                parent: Patch::Ignore,
            },
        )
        .await
        .map_err(|e| e.to_string())?;

    Ok(renamed)
}

pub async fn move_folder(
    store: &Store,
    id: &FolderId,
    target_id: &FolderId,
) -> Result<Folder, String> {
    // Confirm that the parent folder exists.
    store.get_folder_metadata(target_id).await.map_err(|e| {
        if let RepoError::NotFound = e {
            format!("Target folder with id '{}' not found", target_id)
        } else {
            e.to_string()
        }
    })?;

    // Confirm that folder is not already in target
    let folder = store
        .get_folder_metadata(id)
        .await
        .map_err(|e| e.to_string())?;

    if folder.parent.as_ref() == Some(target_id) {
        return Ok(folder);
    }

    // Move the folder
    let moved = store
        .update_folder(
            id,
            PatchFolder {
                name: Patch::Ignore,
                parent: Patch::Set(target_id.clone()),
            },
        )
        .await
        .map_err(|e| e.to_string())?;

    Ok(moved)
}

pub async fn delete_folder(store: &Store, id: &FolderId) -> Result<(), String> {
    unimplemented!();
}
