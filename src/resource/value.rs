mod array;
mod edges;
mod id;
mod object;
mod thing;
pub use array::*;
pub use edges::*;
pub use id::*;
pub use object::*;
use surrealdb::param::PatchOp;
pub use thing::*;

use flutter_rust_bridge::frb;
use surrealdb::sql::json;
use surrealdb::sql::Value;

// * maybe another way
// pub enum ValueEnum{
//     String(String),
// }
#[derive(Clone)]
pub struct ValueMirror(String);

impl Into<Value> for ValueMirror {
    fn into(self) -> Value {
        json(self.0.as_ref()).unwrap()
    }
}

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
