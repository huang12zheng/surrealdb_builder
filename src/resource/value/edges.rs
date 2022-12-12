use flutter_rust_bridge::frb;
use surrealdb::sql::{Dir, Edges, Table, Tables};

use super::IntoThing;

pub struct EdgesMirror {
    pub dir: Dir,
    pub from: String,
    pub what: Tables,
}

pub trait IntoEdgess {
    fn into_edges(self) -> Edges;
}

impl IntoEdgess for EdgesMirror {
    fn into_edges(self) -> Edges {
        Edges {
            dir: self.dir,
            from: self.from.into_thing(),
            what: self.what,
        }
    }
}
#[frb(mirror(Dir))]
pub enum _Dir {
    In,
    Out,
    Both,
}

#[frb(mirror(Tables))]
pub struct _Tables(pub Vec<Table>);
