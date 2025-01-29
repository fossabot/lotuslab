// SPDX-FileCopyrightText: 2025 Jesse Aubin <jesseb34r@jesseaubin.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use std::convert::TryFrom;
use surrealdb::types::{RecordId, RecordIdKey, SurrealValue, ToSql};

use crate::Store;
use crate::repos::user::project::SurrealProjectId;

use lotuslab_types::{List, ListId, ListReadRepo, ListWriteRepo, NewList, PatchList, RepoError};

/// Strongly-typed SurrealDB RecordId for the `list` table
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, SurrealValue)]
#[serde(transparent)]
pub(crate) struct SurrealListId(RecordId);

impl SurrealListId {
    /// *Infallible* constructor for building an ID in code.
    /// Panics if key is not kind `RecordIdKey::String`.
    pub(crate) fn from_key_unchecked(key: impl Into<RecordIdKey>) -> Self {
        let key = key.into();
        debug_assert!(matches!(key, RecordIdKey::String(_)));
        SurrealListId(RecordId::new("list", key))
    }

    /// *Fallible* constructor for parsing from a [RecordId]-ish at runtime.
    /// Checks that the id has table type `list`.
    pub(crate) fn from_record_id(id: impl Into<RecordId>) -> Result<Self, RepoError> {
        let record_id = id.into();
        match record_id.is_table_type(&["list".to_string()]) {
            false => Err(RepoError::InvalidInput("not a list id".into())),
            true => Ok(SurrealListId::from_key_unchecked(record_id.key)),
        }
    }

    /// Convert to a [RecordId] for passing into db queries.
    pub fn into_record(self) -> RecordId {
        self.0
    }

    /// Convert to a string id of form `"list:<key>"`.
    pub fn to_sql(&self) -> String {
        self.0.to_sql()
    }
}

impl TryFrom<RecordId> for SurrealListId {
    type Error = RepoError;

    fn try_from(id: RecordId) -> Result<Self, RepoError> {
        SurrealListId::from_record_id(id)
    }
}

impl TryFrom<ListId> for SurrealListId {
    type Error = RepoError;

    fn try_from(lid: ListId) -> Result<Self, RepoError> {
        let record_id = RecordId::parse_simple(lid.as_ref())
            .map_err(|e| RepoError::InvalidInput(e.to_string()))?;
        SurrealListId::try_from(record_id)
    }
}

impl Into<ListId> for SurrealListId {
    fn into(self) -> ListId {
        ListId(self.to_sql())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, SurrealValue)]
pub(crate) struct SurrealList {
    pub(crate) id: SurrealListId,
    pub(crate) name: String,
    pub(crate) project: SurrealProjectId,
}

impl Into<List> for SurrealList {
    fn into(self) -> List {
        List {
            id: self.id.into(),
            name: self.name,
            project: self.project.into(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, SurrealValue)]
pub(crate) struct SurrealNewList {
    pub(crate) name: String,
    pub(crate) project: SurrealProjectId,
}

impl TryFrom<NewList> for SurrealNewList {
    type Error = RepoError;

    fn try_from(new_list: NewList) -> Result<Self, RepoError> {
        Ok(SurrealNewList {
            name: new_list.name,
            project: new_list.project.try_into()?,
        })
    }
}

#[async_trait]
impl ListReadRepo for Store {
    async fn get_list(&self, id: &ListId) -> Result<List, RepoError> {
        let surreal_id = SurrealListId::try_from(id.clone())?;
        let list = self
            .db
            .select::<Option<SurrealList>>(surreal_id.into_record())
            .await
            .map_err(|e| RepoError::DbError(e.into()))?
            .ok_or(RepoError::NotFound)?;

        Ok(list.into())
    }
}

#[async_trait]
impl ListWriteRepo for Store {
    async fn create_list(&self, new_list: NewList) -> Result<List, RepoError> {
        let created = self
            .db
            .create::<Option<SurrealList>>("list")
            .content(SurrealNewList::try_from(new_list)?)
            .await
            .map_err(|e| RepoError::DbError(e.into()))?
            .ok_or(RepoError::ToDo)?;

        Ok(created.into())
    }

    async fn update_list(&self, id: &ListId, patch: PatchList) -> Result<List, RepoError> {
        let mut content = Vec::new();

        if let lotuslab_types::Patch::Set(name) = &patch.name {
            content.push(format!("name = {}", name));
        }

        if let lotuslab_types::Patch::Set(proj) = &patch.project {
            let surreal_id = SurrealProjectId::try_from(proj.clone())?;
            content.push(format!("project = {}", surreal_id.to_sql()));
        }

        if content.is_empty() {
            return Err(RepoError::NoOp);
        }

        let surreal_id = SurrealListId::try_from(id.clone())?;
        let query = format!("UPDATE {} SET {}", surreal_id.to_sql(), content.join(", "));
        let updated = self
            .db
            .query(query)
            .await
            .map_err(|e| RepoError::DbError(e.into()))?
            .take::<Option<SurrealList>>(0)
            .map_err(|e| RepoError::DbError(e.into()))?
            .ok_or(RepoError::ToDo)?;

        Ok(updated.into())
    }

    async fn delete_list(&self, id: &ListId) -> Result<(), RepoError> {
        let surreal_id = SurrealListId::try_from(id.clone())?;
        self.db
            .delete::<Option<SurrealList>>(surreal_id.into_record())
            .await
            .map_err(|e| RepoError::DbError(e.into()))?
            .ok_or(RepoError::NotFound)?;

        Ok(())
    }
}
