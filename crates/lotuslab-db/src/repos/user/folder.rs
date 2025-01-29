// SPDX-FileCopyrightText: 2025 Jesse Aubin <jesseb34r@jesseaubin.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use serde::{Deserialize, Serialize};
use surrealdb::types::{RecordId, RecordIdKey, SurrealValue, ToSql};

use crate::{Store, repos::user::project::SurrealProject};
use lotuslab_types::{
    Folder, FolderChildren, FolderId, FolderReadRepo, FolderWriteRepo, NewFolder, Patch,
    PatchFolder, Project, RepoError,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, SurrealValue)]
#[serde(transparent)]
pub(crate) struct SurrealFolderId(RecordId);

impl SurrealFolderId {
    /// *Infallible* constructor for building an ID in code.
    /// Panics if key is not kind `RecordIdKey::String`
    pub(crate) fn from_key_unchecked(key: impl Into<RecordIdKey>) -> Self {
        let key = key.into();
        debug_assert!(matches!(key, RecordIdKey::String(_)));

        SurrealFolderId(RecordId::new("folder", key))
    }

    /// *Fallible* constructor for parsing from a [RecordId]-ish at runtime.
    /// Checks that the id has table type `folder`.
    pub(crate) fn from_record_id(id: impl Into<RecordId>) -> Result<Self, RepoError> {
        let record_id = id.into();

        match record_id.is_table_type(&["folder".to_string()]) {
            false => Err(RepoError::InvalidInput("not a folder id".into())),
            true => Ok(SurrealFolderId::from_key_unchecked(record_id.key)),
        }
    }

    /// Convert to a [RecordId] for passing into db queries.
    pub fn into_record(self) -> RecordId {
        self.0
    }

    /// Convert to a string id of form `"folder:id"`.
    pub fn to_sql(&self) -> String {
        self.0.to_sql()
    }
}

impl TryFrom<RecordId> for SurrealFolderId {
    type Error = RepoError;

    fn try_from(id: RecordId) -> Result<Self, RepoError> {
        SurrealFolderId::from_record_id(id)
    }
}

impl TryFrom<FolderId> for SurrealFolderId {
    type Error = RepoError;

    fn try_from(id: FolderId) -> Result<Self, RepoError> {
        let record_id = RecordId::parse_simple(id.as_ref())
            .map_err(|e| RepoError::InvalidInput(e.to_string()))?;
        SurrealFolderId::try_from(record_id)
    }
}

impl Into<FolderId> for SurrealFolderId {
    fn into(self) -> FolderId {
        FolderId(self.to_sql())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, SurrealValue)]
pub(crate) struct SurrealFolder {
    pub(crate) id: SurrealFolderId,
    pub(crate) name: String,
    pub(crate) parent: Option<SurrealFolderId>,
}

impl Into<Folder> for SurrealFolder {
    fn into(self) -> Folder {
        Folder {
            id: self.id.into(),
            name: self.name,
            parent: self.parent.map(|p| p.into()),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, SurrealValue)]
pub(crate) struct SurrealNewFolder {
    pub(crate) name: String,
    pub(crate) parent: SurrealFolderId,
}

impl TryFrom<NewFolder> for SurrealNewFolder {
    type Error = RepoError;

    fn try_from(f: NewFolder) -> Result<Self, RepoError> {
        Ok(SurrealNewFolder {
            name: f.name,
            parent: match f.parent {
                Some(p) => p.try_into()?,
                None => SurrealFolderId::from_key_unchecked("root"),
            },
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, SurrealValue)]
pub(crate) struct SurrealFolderChildren {
    pub(crate) folders: Vec<SurrealFolder>,
    pub(crate) projects: Vec<SurrealProject>,
}

impl Into<FolderChildren> for SurrealFolderChildren {
    fn into(self) -> FolderChildren {
        FolderChildren {
            folders: self
                .folders
                .into_iter()
                .map(|folder| folder.into())
                .collect::<Vec<Folder>>(),
            projects: self
                .projects
                .into_iter()
                .map(|project| project.into())
                .collect::<Vec<Project>>(),
        }
    }
}

#[async_trait::async_trait]
impl FolderReadRepo for Store {
    async fn get_folder_metadata(&self, id: &FolderId) -> Result<Folder, RepoError> {
        let surreal_id = SurrealFolderId::try_from(id.clone())?;
        let folder = self
            .db
            .select::<Option<SurrealFolder>>(surreal_id.into_record())
            .await
            .map_err(|e| RepoError::DbError(e.into()))?
            .ok_or(RepoError::NotFound)?;

        Ok(folder.into())
    }

    async fn get_folder_children(&self, id: &FolderId) -> Result<FolderChildren, RepoError> {
        let surreal_id = SurrealFolderId::try_from(id.clone())?;

        let mut response = self
            .db
            .query("SELECT <~folder.* AS folders, <~project.* AS projects FROM $id;")
            .bind(("id", surreal_id.into_record()))
            .await
            .map_err(|e| RepoError::DbError(e.into()))?;

        let children = response
            .take::<Option<SurrealFolderChildren>>(0)
            .map_err(|e| RepoError::DbError(e.into()))?
            .ok_or(RepoError::NotFound)?;

        Ok(children.into())
    }
}

#[async_trait::async_trait]
impl FolderWriteRepo for Store {
    async fn create_folder(&self, new_folder: NewFolder) -> Result<Folder, RepoError> {
        let created = self
            .db
            .create::<Option<SurrealFolder>>("folder")
            .content(SurrealNewFolder::try_from(new_folder)?)
            .await
            .map_err(|e| RepoError::DbError(e.into()))?
            .ok_or(RepoError::ToDo)?;

        Ok(created.into())
    }

    async fn update_folder(&self, id: &FolderId, patch: PatchFolder) -> Result<Folder, RepoError> {
        let mut content = Vec::new();

        if let Patch::Set(name) = &patch.name {
            content.push(format!("name = {}", name));
        }

        if let Patch::Set(parent) = &patch.parent {
            let surreal_id = SurrealFolderId::try_from(parent.clone())?;
            content.push(format!("parent = {}", surreal_id.to_sql()));
        }

        if content.is_empty() {
            return Err(RepoError::NoOp);
        }

        let surreal_id = SurrealFolderId::try_from(id.clone())?;
        let query = format!(
            "UPDATE folder:{} SET {}",
            surreal_id.to_sql(),
            content.join(", ")
        );
        let updated = self
            .db
            .query(query)
            .await
            .map_err(|e| RepoError::DbError(e.into()))?
            .take::<Option<SurrealFolder>>(0)
            .map_err(|e| RepoError::DbError(e.into()))?
            .ok_or(RepoError::ToDo)?;

        Ok(updated.into())
    }

    // TODO figure out what happens to project contained in folders.
    async fn delete_folder(&self, id: &FolderId) -> Result<(), RepoError> {
        let surreal_id = SurrealFolderId::try_from(id.clone())?;
        self.db
            .delete::<Option<SurrealFolder>>(surreal_id.into_record())
            .await
            .map_err(|e| RepoError::DbError(e.into()))?
            .ok_or(RepoError::NotFound)?;

        Ok(())
    }
}
