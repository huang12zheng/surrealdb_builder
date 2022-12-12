use crate::resource::IdMirror;
use surrealdb::sql::Thing;

pub struct ThingMirror {
    pub tb: String,
    pub id: IdMirror,
}

impl Into<Thing> for ThingMirror {
    fn into(self) -> Thing {
        Thing {
            tb: self.tb,
            id: self.id.into(),
        }
    }
}
