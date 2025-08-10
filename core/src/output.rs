use std::{collections::HashMap, string};

use logic::{
    id::{SpaceTimeId, points::Point},
    set::SpaceTimeIdSet,
};
use schemars::JsonSchema;
use serde::Serialize;

#[derive(Serialize, JsonSchema)]
pub struct GetValueOutput {
    pub spacetimeid: SpaceTimeId,
    pub id_string: Option<String>,
    pub vertex: Option<[Point; 8]>,
    pub center: Option<Point>,
    pub value: ValueEntry,
}

#[derive(Serialize, JsonSchema)]
pub struct SelectOutPut {
    pub spacetimeid: SpaceTimeId,
    pub id_string: Option<String>,
    pub vertex: Option<[Point; 8]>,
    pub center: Option<Point>,
}

use crate::io::ValueEntry;
#[derive(Serialize, JsonSchema)]
pub enum Output {
    SpaceNames(Vec<String>),
    KeyNames(Vec<String>),
    GetValue(Vec<GetValueOutput>),
    SelectValue(Vec<SelectOutPut>),
    SpaceTimeIdSet(SpaceTimeIdSet),
    Version(String),
    Success,
}
