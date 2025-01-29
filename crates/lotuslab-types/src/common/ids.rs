// SPDX-FileCopyrightText: 2025 Jesse Aubin <jesseb34r@jesseaubin.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use serde::{Deserialize, Serialize};
use specta::Type;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
pub struct FolderId(pub String);
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
pub struct ProjectId(pub String);
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
pub struct ListId(pub String);
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
pub struct ListItemId(pub String);
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
pub struct TagId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
pub struct CardCoreId(pub String);
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
pub struct CardPrintingId(pub String);
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
pub struct SetId(pub String);
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
pub struct ArtistId(pub String);

impl From<String> for FolderId {
    fn from(value: String) -> Self {
        FolderId(value)
    }
}
impl From<&str> for FolderId {
    fn from(value: &str) -> Self {
        FolderId(value.to_string())
    }
}
impl fmt::Display for FolderId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
impl AsRef<str> for FolderId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
impl From<FolderId> for String {
    fn from(value: FolderId) -> Self {
        value.0
    }
}

impl From<String> for ProjectId {
    fn from(value: String) -> Self {
        ProjectId(value)
    }
}
impl From<&str> for ProjectId {
    fn from(value: &str) -> Self {
        ProjectId(value.to_string())
    }
}
impl fmt::Display for ProjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
impl AsRef<str> for ProjectId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
impl From<ProjectId> for String {
    fn from(value: ProjectId) -> Self {
        value.0
    }
}

impl From<String> for ListId {
    fn from(value: String) -> Self {
        ListId(value)
    }
}
impl From<&str> for ListId {
    fn from(value: &str) -> Self {
        ListId(value.to_string())
    }
}
impl fmt::Display for ListId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
impl AsRef<str> for ListId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
impl From<ListId> for String {
    fn from(value: ListId) -> Self {
        value.0
    }
}

impl From<String> for ListItemId {
    fn from(value: String) -> Self {
        ListItemId(value)
    }
}
impl From<&str> for ListItemId {
    fn from(value: &str) -> Self {
        ListItemId(value.to_string())
    }
}
impl fmt::Display for ListItemId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
impl AsRef<str> for ListItemId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
impl From<ListItemId> for String {
    fn from(value: ListItemId) -> Self {
        value.0
    }
}

impl From<String> for TagId {
    fn from(value: String) -> Self {
        TagId(value)
    }
}
impl From<&str> for TagId {
    fn from(value: &str) -> Self {
        TagId(value.to_string())
    }
}
impl fmt::Display for TagId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
impl AsRef<str> for TagId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
impl From<TagId> for String {
    fn from(value: TagId) -> Self {
        value.0
    }
}

impl From<String> for CardCoreId {
    fn from(value: String) -> Self {
        CardCoreId(value)
    }
}
impl From<&str> for CardCoreId {
    fn from(value: &str) -> Self {
        CardCoreId(value.to_string())
    }
}
impl fmt::Display for CardCoreId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
impl AsRef<str> for CardCoreId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
impl From<CardCoreId> for String {
    fn from(value: CardCoreId) -> Self {
        value.0
    }
}

impl From<String> for CardPrintingId {
    fn from(value: String) -> Self {
        CardPrintingId(value)
    }
}
impl From<&str> for CardPrintingId {
    fn from(value: &str) -> Self {
        CardPrintingId(value.to_string())
    }
}
impl fmt::Display for CardPrintingId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
impl AsRef<str> for CardPrintingId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
impl From<CardPrintingId> for String {
    fn from(value: CardPrintingId) -> Self {
        value.0
    }
}

impl From<String> for SetId {
    fn from(value: String) -> Self {
        SetId(value)
    }
}
impl From<&str> for SetId {
    fn from(value: &str) -> Self {
        SetId(value.to_string())
    }
}
impl fmt::Display for SetId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
impl AsRef<str> for SetId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
impl From<SetId> for String {
    fn from(value: SetId) -> Self {
        value.0
    }
}

impl From<String> for ArtistId {
    fn from(value: String) -> Self {
        ArtistId(value)
    }
}
impl From<&str> for ArtistId {
    fn from(value: &str) -> Self {
        ArtistId(value.to_string())
    }
}
impl fmt::Display for ArtistId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
impl AsRef<str> for ArtistId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
impl From<ArtistId> for String {
    fn from(value: ArtistId) -> Self {
        value.0
    }
}
