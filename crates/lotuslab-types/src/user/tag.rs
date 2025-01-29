// SPDX-FileCopyrightText: 2025 Jesse Aubin <jesseb34r@jesseaubin.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use serde::{Deserialize, Serialize};

use crate::{Patch, RepoError, TagId};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tag {
    pub id: TagId,
    pub name: String,
    pub color: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewTag {
    pub name: String,
    pub color: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateTag {
    #[serde(default)]
    pub name: Patch<String>,
    #[serde(default)]
    pub color: Patch<String>,
}

#[async_trait::async_trait]
pub trait TagReadRepo: Send + Sync {
    async fn get_tag(&self, new: NewTag) -> Result<Tag, RepoError>;
}

#[async_trait::async_trait]
pub trait TagWriteRepo: Send + Sync {
    async fn create_tag(&self, new: NewTag) -> Result<Tag, RepoError>;
    async fn update_tag(&self, id: &TagId, patch: UpdateTag) -> Result<Tag, RepoError>;
    async fn delete_tag(&self, id: &TagId) -> Result<(), RepoError>;
}
