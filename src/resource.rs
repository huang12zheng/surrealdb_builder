mod range;
pub use range::*;
mod value;
pub use value::*;

use surrealdb::{
    param::Resource,
    sql::{Array, Edges, Object, Table, Thing},
};

#[derive(Clone)]
pub enum ResourceMirror {
    // StringId((String, Id)), use Thing please
    OptionObject(ObjectMirror),
    // , use String("{one:1,two:2,tre:3+1}") pleasse
    VecArray(Vec<ValueMirror>),
    VecEdges(Edges),
    VecString(String),
    VecTable(Table),
    OptionThing(Thing),
}

impl<R> Resource<R> for ResourceMirror {
    // use match_template::*;
    /// match_template! {
    ///     T = [VecArray,VecEdges,VecString,VecTable,OptionThing],
    ///     match self {
    ///         ResourceEnum::T(res) => res.into_db_resource()
    ///     }
    /// }
    fn into_db_resource(self) -> surrealdb::Result<surrealdb::param::DbResource> {
        match self {
            ResourceMirror::VecArray(res) => Resource::<Vec<R>>::into_db_resource(res.into_array()),
            ResourceMirror::VecEdges(res) => Resource::<Vec<R>>::into_db_resource(res),
            ResourceMirror::VecString(res) => Resource::<Vec<R>>::into_db_resource(res),
            ResourceMirror::VecTable(res) => Resource::<Vec<R>>::into_db_resource(res),
            ResourceMirror::OptionThing(res) => Resource::<Option<R>>::into_db_resource(res),
            ResourceMirror::OptionObject(res) => {
                Resource::<Option<R>>::into_db_resource(Into::<Object>::into(res))
            }
        }
    }
}
