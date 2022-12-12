use std::ops::Bound;

use surrealdb::sql::Id;

use super::{IntoArray, ValueMirror};

/// [Id]
/// Object(Object), not support
#[derive(Clone)]
pub enum IdMirror {
    Number(i64),
    String(String),
    Array(Vec<ValueMirror>),
    Unbounded,
}

impl Into<Id> for IdMirror {
    fn into(self) -> Id {
        match self {
            IdMirror::Number(v) => v.into(),
            IdMirror::String(v) => v.into(),
            IdMirror::Array(v) => v.into_array().into(),
            _ => unreachable!(),
        }
    }
}
