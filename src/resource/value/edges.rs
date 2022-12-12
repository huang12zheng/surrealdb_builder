use flutter_rust_bridge::frb;
use surrealdb::sql::{Table, Tables};

use super::thing::ThingMirror;

pub struct Edges {
    pub dir: Dir,
    pub from: ThingMirror,
    pub what: Tables,
}

pub enum Dir {
    In,
    Out,
    Both,
}

#[frb(mirror(Tables))]
pub struct _Tables(pub Vec<Table>);
