use std::ops::Bound;

use super::{IdMirror, IntoArray};

use surrealdb::{param::Range, sql::Id};

#[derive(Clone)]
pub struct RangeMirror {
    pub start: IdMirror,
    pub end: IdMirror,
}

pub struct StringRange {
    pub start: String,
    pub end: String,
}

impl Into<Range<Id>> for RangeMirror {
    fn into(self) -> Range<Id> {
        (self.start.into(), self.end.into()).into()
    }
}
impl Into<Range<Id>> for StringRange {
    fn into(self) -> Range<Id> {
        (
            Bound::<Id>::Included(self.start.into()),
            Bound::<Id>::Included(self.end.into()),
        )
            .into()
    }
}

impl Into<Bound<Id>> for IdMirror {
    fn into(self) -> Bound<Id> {
        match self {
            IdMirror::Number(v) => Bound::Included(v.into()),
            IdMirror::String(v) => Bound::Included(v.into()),
            IdMirror::Array(v) => Bound::Included(v.into_array().into()),
            IdMirror::Unbounded => Bound::Unbounded,
        }
    }
}
