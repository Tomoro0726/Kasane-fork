use std::{collections::HashMap, string};

use logic::{
    id::{SpaceTimeId, points::Point},
    set::SpaceTimeIdSet,
};
use schemars::JsonSchema;
use serde::Serialize;

#[derive(Serialize, JsonSchema)]
pub struct GetValueOutput {
    pub id: String,
    pub spacetimeid: SpaceTimeId,
    pub vertex: [Point; 8],
    pub value: ValueEntry,
}

#[derive(Serialize, JsonSchema)]
pub struct SelectOutPut {
    pub id: String,
    pub spacetimeid: SpaceTimeId,
    pub vertex: [Point; 8],
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
