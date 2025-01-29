// SPDX-FileCopyrightText: 2025 Jesse Aubin <jesseb34r@jesseaubin.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepoError {
    #[error("invalid input: {0}")]
    InvalidInput(String),
    #[error("not found")]
    NotFound,
    #[error("noop")]
    NoOp,
    #[error("todo")]
    ToDo, // implement helpful errors here later
    #[error(transparent)]
    DbError(#[from] anyhow::Error),
}
