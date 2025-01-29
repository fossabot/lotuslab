// SPDX-FileCopyrightText: 2025 Jesse Aubin <jesseb34r@jesseaubin.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use specta::Type;

/// IMPORTANT: Use `#[serde(default)]` on any field with type `Patch<T>` to
/// correctly handle omitted properties being set to `Patch::Ignore`
///
/// `Patch<T>` is a utility enum representing modifications to a value:
/// - `Ignore`: no change, field is omitted in serialization (None)
/// - `Clear`: explicitly clear the value, serialized as Some(None) (null in JSON)
/// - `Set(T)`: set the value to `T`, serialized as Some(Some(T))
///
/// Custom Serialize/Deserialize implementations convert between `Patch<T>`
/// and `Option<Option<T>>` to provide an idiomatic JSON representation for
/// the javascript frontend, where absent fields mean ignore, null means clear,
/// and any value means set.
#[derive(Clone, Debug, Type)]
pub enum Patch<T> {
    Ignore,
    Clear,
    Set(T),
}

impl<T> Default for Patch<T> {
    fn default() -> Self {
        return Patch::Ignore;
    }
}

// Serialize Patch<T> as Option<Option<T>>:
//   Ignore => None
//   Clear  => Some(None)
//   Set(v) => Some(Some(v))
impl<T> Serialize for Patch<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        return match self {
            Patch::Ignore => Option::<Option<&T>>::None.serialize(serializer),
            Patch::Clear => Some::<Option<&T>>(None).serialize(serializer),
            Patch::Set(v) => Some::<Option<&T>>(Some(v)).serialize(serializer),
        };
    }
}

impl<'de, T> Deserialize<'de> for Patch<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt: Option<Option<T>> = Option::deserialize(deserializer)?;
        return Ok(match opt {
            None => Patch::Ignore,
            Some(None) => Patch::Clear,
            Some(Some(v)) => Patch::Set(v),
        });
    }
}
