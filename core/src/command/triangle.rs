use kasane_logic::{function::triangle::triangle as other_triangle, set::SpaceTimeIdSet};

use crate::{error::Error, output::Output, parser::Triangle};

pub fn triangle(v: Triangle) -> SpaceTimeIdSet {
    other_triangle(v.zoom, v.point1, v.point2, v.point3)
}
