use flutter_rust_bridge::frb;
use surrealdb::sql::{Dir, Edges, Table, Tables};

use super::{IntoTables, IntoThing};

#[derive(Default)]
pub struct EdgesMirror {
    pub dir: Dir,
    pub from: String,
    pub what: Vec<String>,
}

pub trait IntoEdges {
    fn into_edges(self) -> Edges;
}

impl IntoEdges for EdgesMirror {
    fn into_edges(self) -> Edges {
        Edges {
            dir: self.dir,
            from: self.from.into_thing(),
            what: self.what.into_tables(),
        }
    }
}
#[frb(mirror(Dir))]
pub enum _Dir {
    In,
    Out,
    Both,
}
