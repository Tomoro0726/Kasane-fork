use kasane_logic::{function::line::line as other_line, set::SpaceTimeIdSet};
use serde::de::value::Error;

use crate::{output::Output, parser::Line};

pub fn line(v: Line) -> SpaceTimeIdSet {
    other_line(v.zoom, v.start, v.end)
}
