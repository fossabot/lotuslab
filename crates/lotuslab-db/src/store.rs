// SPDX-FileCopyrightText: 2025 Jesse Aubin <jesseb34r@jesseaubin.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! SurrealStore — a small, crate-local abstraction for constructing and managing
//! a SurrealDB-backed store used by repo implementations.
//!
//! This abstraction provides two storage backends:
//! - In-memory storage for testing and development
//! - RocksDB storage for production persistence

use std::path::Path;
use surrealdb::{
    Surreal,
    engine::local::{Db, RocksDb},
    opt::{
        Config,
        auth::Root,
        capabilities::{Capabilities, ExperimentalFeature},
    },
};

pub struct Store {
    pub(crate) db: Surreal<Db>,
}

impl Store {
    /// Creates an in-memory SurrealDB instance for testing. Do not use in production code.
    #[cfg(test)]
    pub(crate) async fn new_mem() -> surrealdb::Result<Self> {
        let db = Surreal::new::<surrealdb::engine::local::Mem>(()).await?;

        let _version = db.version().await;
        debug_assert!(
            _version.is_ok(),
            "Database connection should be valid after initialization"
        );

        Ok(Self { db })
    }

    /// Creates a RocksDB-backed SurrealDB instance for production use.
    ///
    /// The path must be a valid directory path where RocksDB files will be stored.
    /// The directory will be created if it doesn't exist.
    pub(crate) async fn new_rocksdb(path: impl AsRef<Path>) -> surrealdb::Result<Self> {
        let path_ref = path.as_ref();

        debug_assert!(
            !path_ref.as_os_str().is_empty(),
            "RocksDB path must not be empty"
        );

        let config = Config::default()
            .capabilities(
                Capabilities::all()
                    .with_experimental_features_allowed(&[ExperimentalFeature::RecordReferences]),
            )
            .user(Root {
                username: "root".to_string(),
                password: "password".to_string(),
            });

        let path_buf = path_ref.to_path_buf();
        let db = Surreal::new::<RocksDb>((path_buf, config)).await?;

        let _version = db.version().await;
        debug_assert!(
            _version.is_ok(),
            "Database connection should be valid after initialization"
        );

        db.signin(Root {
            username: "root".to_string(),
            password: "password".to_string(),
        })
        .await?;

        db.use_ns("lotuslab").use_db("db").await?;

        return Ok(Self { db });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_memory_store_creation() -> surrealdb::Result<()> {
        // Creating an in-memory SurrealStore should succeed.
        let _store = Store::new_mem().await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_rocksdb_store_creation_and_cleanup() -> surrealdb::Result<()> {
        let temp_dir = tempdir().expect("Failed to create temporary directory");
        let db_path = temp_dir.path().to_path_buf();

        // Creating a RocksDb-backed SurrealStore should succeed.
        let store = Store::new_rocksdb(&db_path).await?;
        assert!(
            db_path.exists(),
            "RocksDB should create files at the specified path"
        );

        drop(store);
        temp_dir
            .close()
            .expect("Failed to cleanup temporary directory");

        Ok(())
    }
}
