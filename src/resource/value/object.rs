use surrealdb::sql::{json, Object, Value};

#[derive(Clone)]
pub struct ObjectMirror(String);
impl Into<Object> for ObjectMirror {
    fn into(self) -> Object {
        match json(self.0.as_ref()).unwrap() {
            Value::Object(obj) => obj,
            _ => unreachable!("only supported Object"),
        }
    }
}
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
