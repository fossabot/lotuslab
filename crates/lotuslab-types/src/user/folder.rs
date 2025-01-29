// SPDX-FileCopyrightText: 2025 Jesse Aubin <jesseb34r@jesseaubin.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use serde::{Deserialize, Serialize};
use specta::Type;

use crate::{FolderId, Patch, Project, RepoError};

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
pub struct Folder {
    pub id: FolderId,
    pub name: String,
    pub parent: Option<FolderId>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
pub struct NewFolder {
    pub name: String,
    pub parent: Option<FolderId>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
pub struct PatchFolder {
    #[serde(default)]
    pub name: Patch<String>,
    #[serde(default)]
    pub parent: Patch<FolderId>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
pub struct FolderChildren {
    pub folders: Vec<Folder>,
    pub projects: Vec<Project>,
}

#[async_trait::async_trait]
pub trait FolderReadRepo: Send + Sync {
    async fn get_folder_metadata(&self, id: &FolderId) -> Result<Folder, RepoError>;
    async fn get_folder_children(&self, id: &FolderId) -> Result<FolderChildren, RepoError>;
}

#[async_trait::async_trait]
pub trait FolderWriteRepo: Send + Sync {
    async fn create_folder(&self, new: NewFolder) -> Result<Folder, RepoError>;
    async fn update_folder(&self, id: &FolderId, patch: PatchFolder) -> Result<Folder, RepoError>;
    async fn delete_folder(&self, id: &FolderId) -> Result<(), RepoError>;
}
