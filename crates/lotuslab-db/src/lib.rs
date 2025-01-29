// SPDX-FileCopyrightText: 2025 Jesse Aubin <jesseb34r@jesseaubin.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::path::Path;

use lotuslab_types::RepoError;

mod repos;
mod store;
mod tests;

pub use store::Store;

const SCHEMA: &str = include_str!("schema.surql");

pub async fn open_store(path: impl AsRef<Path>) -> Result<Store, RepoError> {
    let store = Store::new_rocksdb(path)
        .await
        .map_err(|e| RepoError::DbError(e.into()))?;

    store
        .db
        .query(SCHEMA)
        .await
        .map_err(|e| RepoError::DbError(e.into()))?;

    return Ok(store);
}
