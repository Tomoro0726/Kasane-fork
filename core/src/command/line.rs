use crate::json::input::Line;
use kasane_logic::{function::line::line as other_line, set::SpaceTimeIdSet};

pub fn line(v: Line) -> SpaceTimeIdSet {
    other_line(v.zoom, v.start, v.end)
}
