use surrealdb::sql::{json, Object, Value};

pub trait IntoObject {
    fn into_object(self) -> Object;
}

impl IntoObject for String {
    fn into_object(self) -> Object {
        match json(self.as_str()).unwrap() {
            Value::Object(obj) => obj,
            _ => unreachable!("Only supported Object"),
        }
    }
}
