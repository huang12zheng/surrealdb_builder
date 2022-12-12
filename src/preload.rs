// pub use crate::api::create::create;
pub use crate::lazy::DB;
pub use crate::resource::{
    EdgesMirror, IntoArray, IntoEdges, IntoObject, IntoThing, IntoValue, StringRange,
};
pub use crate::{create, delete, patch, select, update};
pub use serde::{Deserialize, Serialize};
pub use surrealdb::sql::Table;
