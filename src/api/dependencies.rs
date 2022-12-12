pub use crate::lazy::DB;
pub use crate::resource::{IntoArray, IntoObject, IntoThing, IntoValue, StringRange};
pub use serde::{Deserialize, Serialize};
pub use surrealdb::{
    param::PatchOp,
    sql::{Edges, Table, Thing},
};

pub struct Add {
    pub path: String,
    pub value: String,
}
pub struct Remove {
    pub path: String,
}
pub struct Replace {
    pub path: String,
    pub value: String,
}

pub enum InnerOp {
    Add(Add),
    Remove(Remove),
    Replace(Replace),
}

pub trait IntoPatchOp {
    fn into_patch_op(self) -> PatchOp;
}

impl IntoPatchOp for InnerOp {
    fn into_patch_op(self) -> PatchOp {
        match self {
            InnerOp::Add(Add { path, value }) => PatchOp::add(&path, value.into_value()),
            InnerOp::Remove(Remove { path }) => PatchOp::remove(&path),
            InnerOp::Replace(Replace { path, value }) => {
                PatchOp::replace(&path, value.into_value())
            }
        }
    }
}
