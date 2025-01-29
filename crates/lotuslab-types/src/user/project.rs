// SPDX-FileCopyrightText: 2025 Jesse Aubin <jesseb34r@jesseaubin.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use serde::{Deserialize, Serialize};
use specta::Type;

use crate::{FolderId, Patch, ProjectId, RepoError};

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
pub struct Project {
    pub id: ProjectId,
    pub name: String,
    pub folder: FolderId,
}

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
pub struct NewProject {
    pub name: String,             // TODO allow random project name?
    pub folder: Option<FolderId>, // TODO encode default root folder in type?
}

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
pub struct PatchProject {
    #[serde(default)]
    pub name: Patch<String>,
    #[serde(default)]
    pub folder: Patch<FolderId>,
}

// TODO figure out how to assert invariants for patches

// Repos
#[async_trait::async_trait]
pub trait ProjectReadRepo: Send + Sync {
    async fn get_project(&self, id: &ProjectId) -> Result<Project, RepoError>;
}

#[async_trait::async_trait]
pub trait ProjectWriteRepo: Send + Sync {
    async fn create_project(&self, new: NewProject) -> Result<Project, RepoError>;
    async fn update_project(
        &self,
        id: &ProjectId,
        patch: PatchProject,
    ) -> Result<Project, RepoError>;
    async fn delete_project(&self, id: &ProjectId) -> Result<(), RepoError>;
}
