#[frb(mirror(Table))]
pub struct _Table(pub String);

#[frb(mirror(Dir))]
pub enum _Dir {
    In,
    Out,
    Both,
}

#[frb(mirror(StringRange))]
pub struct _StringRange {
    pub start: String,
    pub end: String,
}

#[frb(mirror(EdgesMirror))]
pub struct _EdgesMirror {
    pub dir: Dir,
    pub from: String,
    pub what: Vec<Table>,
}
