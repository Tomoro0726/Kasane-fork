use kasane_logic::function::line::line as other_line;
use serde::de::value::Error;

use crate::{output::Output, parser::Line};

pub fn line(v: Line) -> Result<Output, Error> {
    Ok(Output::SpaceTimeIdSet(other_line(v.zoom, v.start, v.end)))
}
