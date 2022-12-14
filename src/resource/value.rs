mod array;
mod edges;
mod object;
pub use array::*;
pub use edges::*;
pub use object::*;
use surrealdb::sql::{json, thing, Table, Tables, Thing, Value};

use flutter_rust_bridge::frb;

#[frb(mirror(Table))]
pub struct _Table(pub String);

pub trait IntoValue {
    fn into_value(self) -> Value;
}

impl IntoValue for String {
    fn into_value(self) -> Value {
        json(&self).unwrap()
    }
}
pub trait IntoThing {
    fn into_thing(self) -> Thing;
}

impl IntoThing for String {
    fn into_thing(self) -> Thing {
        thing(&self).unwrap()
    }
}

impl IntoThing for (&str, String) {
    fn into_thing(self) -> Thing {
        Thing {
            tb: self.0.to_owned(),
            id: self.1.into(),
        }
    }
}

pub trait IntoTable {
    fn into_table(self) -> Table;
}

impl IntoTable for String {
    fn into_table(self) -> Table {
        Table(self)
    }
}
impl IntoTable for &str {
    fn into_table(self) -> Table {
        Table(self.to_string())
    }
}
pub trait IntoTables {
    fn into_tables(self) -> Tables;
}

impl IntoTables for Vec<String> {
    fn into_tables(self) -> Tables {
        Tables(self.into_iter().map(|e| e.into_table()).collect())
    }
}
