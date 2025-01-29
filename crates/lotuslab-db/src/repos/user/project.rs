// SPDX-FileCopyrightText: 2025 Jesse Aubin <jesseb34r@jesseaubin.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};
use surrealdb::types::{RecordId, RecordIdKey, SurrealValue, ToSql};

use crate::{Store, repos::user::folder::SurrealFolderId};
use lotuslab_types::{
    NewProject, Patch, PatchProject, Project, ProjectId, ProjectReadRepo, ProjectWriteRepo,
    RepoError,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, SurrealValue)]
#[serde(transparent)]
pub(crate) struct SurrealProjectId(RecordId);

impl SurrealProjectId {
    /// *Infallible* constructor for building an ID in code.
    /// Panics if key is not kind `RecordIdKey::String`
    pub(crate) fn from_key_unchecked(key: impl Into<RecordIdKey>) -> Self {
        let key = key.into();
        debug_assert!(matches!(key, RecordIdKey::String(_)));

        SurrealProjectId(RecordId::new("project", key))
    }

    /// *Fallible* constructor for parsing from a [RecordId]-ish at runtime.
    /// Checks that the id has table type `project`.
    pub(crate) fn from_record_id(id: impl Into<RecordId>) -> Result<Self, RepoError> {
        let record_id = id.into();

        match record_id.is_table_type(&["project".to_string()]) {
            false => Err(RepoError::InvalidInput("not a project id".into())),
            true => Ok(SurrealProjectId::from_key_unchecked(record_id.key)),
        }
    }

    /// Convert to a [RecordId] for passing into db queries.
    pub fn into_record(self) -> RecordId {
        self.0
    }

    /// Convert to a string id of form `"project:id"`.
    pub fn to_sql(&self) -> String {
        self.0.to_sql()
    }
}

impl TryFrom<RecordId> for SurrealProjectId {
    type Error = RepoError;

    fn try_from(id: RecordId) -> Result<Self, RepoError> {
        SurrealProjectId::from_record_id(id)
    }
}

impl TryFrom<ProjectId> for SurrealProjectId {
    type Error = RepoError;

    fn try_from(id: ProjectId) -> Result<Self, RepoError> {
        let record_id = RecordId::parse_simple(id.as_ref())
            .map_err(|e| RepoError::InvalidInput(e.to_string()))?;
        SurrealProjectId::try_from(record_id)
    }
}

impl Into<ProjectId> for SurrealProjectId {
    fn into(self) -> ProjectId {
        ProjectId(self.to_sql())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, SurrealValue)]
pub(crate) struct SurrealProject {
    pub(crate) id: SurrealProjectId,
    pub(crate) name: String,
    pub(crate) folder: SurrealFolderId,
}

impl Into<Project> for SurrealProject {
    fn into(self) -> Project {
        Project {
            id: self.id.into(),
            name: self.name,
            folder: self.folder.into(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, SurrealValue)]
pub(crate) struct SurrealNewProject {
    pub(crate) name: String,
    pub(crate) folder: SurrealFolderId,
}

impl TryFrom<NewProject> for SurrealNewProject {
    type Error = RepoError;

    fn try_from(new_project: NewProject) -> Result<Self, RepoError> {
        Ok(SurrealNewProject {
            name: new_project.name,
            folder: match new_project.folder {
                Some(f) => f.try_into()?,
                None => SurrealFolderId::from_key_unchecked("root"),
            },
        })
    }
}

#[async_trait]
impl ProjectReadRepo for Store {
    async fn get_project(&self, id: &ProjectId) -> Result<Project, RepoError> {
        let surreal_id = SurrealProjectId::try_from(id.clone())?;

        let project = self
            .db
            .select::<Option<SurrealProject>>(surreal_id.into_record())
            .await
            .map_err(|e| RepoError::DbError(e.into()))?
            .ok_or(RepoError::NotFound)?;

        Ok(project.into())
    }
}

#[async_trait]
impl ProjectWriteRepo for Store {
    async fn create_project(&self, new_project: NewProject) -> Result<Project, RepoError> {
        let content = SurrealNewProject::try_from(new_project)?;

        let created = self
            .db
            .create::<Option<SurrealProject>>("project")
            .content(content)
            .await
            .map_err(|e| RepoError::DbError(e.into()))?
            .ok_or(RepoError::ToDo)?;

        Ok(created.into())
    }

    async fn update_project(
        &self,
        id: &ProjectId,
        patch: PatchProject,
    ) -> Result<Project, RepoError> {
        let mut content = Vec::new();

        if let Patch::Set(name) = &patch.name {
            content.push(format!("name = {}", name));
        }

        if let Patch::Set(folder) = &patch.folder {
            let id = SurrealFolderId::try_from(folder.clone())?;
            content.push(format!("folder = {}", id.to_sql()));
        }

        if content.is_empty() {
            return Err(RepoError::NoOp);
        }

        let surreal_id = SurrealProjectId::try_from(id.clone())?;
        let query = format!("UPDATE {} SET {}", surreal_id.to_sql(), content.join(", "));

        let updated = self
            .db
            .query(query)
            .await
            .map_err(|e| RepoError::DbError(e.into()))?
            .take::<Option<SurrealProject>>(0)
            .map_err(|e| RepoError::DbError(e.into()))?
            .ok_or(RepoError::ToDo)?;

        Ok(updated.into())
    }

    async fn delete_project(&self, id: &ProjectId) -> Result<(), RepoError> {
        let surreal_id = SurrealProjectId::try_from(id.clone())?;
        self.db
            .delete::<Option<SurrealProject>>(surreal_id.into_record())
            .await
            .map_err(|e| RepoError::DbError(e.into()))?
            .ok_or(RepoError::NotFound)?;

        Ok(())
    }
}
