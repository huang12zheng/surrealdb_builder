pub use super::{IDb, DB};
pub use crate::resource::{IntoArray, IntoObject, IntoValue, StringRange};
pub use serde::{Deserialize, Serialize};
pub use surrealdb::{
    param::PatchOp,
    sql::{Edges, Table, Thing, Value},
};
#[derive(Debug, Serialize)]
#[serde(tag = "op", rename_all = "lowercase")]
pub enum InnerOp {
    Add { path: String, value: String },
    Remove { path: String },
    Replace { path: String, value: String },
}

pub trait IntoPatchOp {
    fn into_patch_op(self) -> PatchOp;
}
