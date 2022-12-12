use std::ops::Bound;

use surrealdb::{param::Range, sql::Id};

pub struct StringRange {
    pub start: String,
    pub end: String,
}

impl From<StringRange> for Range<Id> {
    fn from(val: StringRange) -> Self {
        (
            Bound::<Id>::Included(val.start.into()),
            Bound::<Id>::Included(val.end.into()),
        )
            .into()
    }
}
