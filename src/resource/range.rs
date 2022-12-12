use std::ops::Bound;

use surrealdb::{param::Range, sql::Id};

pub struct StringRange {
    pub start: String,
    pub end: String,
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
