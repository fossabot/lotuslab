// SPDX-FileCopyrightText: 2025 Jesse Aubin <jesseb34r@jesseaubin.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use serde::{Deserialize, Serialize};
use specta::Type;

use crate::{ListId, Patch, ProjectId, RepoError};

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
pub struct List {
    pub id: ListId,
    pub name: String,
    pub project: ProjectId,
}

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
pub struct NewList {
    pub name: String,
    pub project: ProjectId,
}

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
pub struct PatchList {
    #[serde(default)]
    pub name: Patch<String>,
    #[serde(default)]
    pub project: Patch<ProjectId>,
}

#[async_trait::async_trait]
pub trait ListReadRepo: Send + Sync {
    async fn get_list(&self, id: &ListId) -> Result<List, RepoError>;
    // TODO figure out the best way to get a formatted list for the frontend
}

#[async_trait::async_trait]
pub trait ListWriteRepo: Send + Sync {
    async fn create_list(&self, new: NewList) -> Result<List, RepoError>;
    async fn update_list(&self, id: &ListId, patch: PatchList) -> Result<List, RepoError>;
    async fn delete_list(&self, id: &ListId) -> Result<(), RepoError>;
}
