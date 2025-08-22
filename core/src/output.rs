use kasane_logic::{
    id::{SpaceTimeId, coordinates::Point},
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
pub struct SelectOutput {
    pub spacetimeid: SpaceTimeId,
    pub id_string: Option<String>,
    pub vertex: Option<[Point; 8]>,
    pub center: Option<Point>,
}

#[derive(Serialize, JsonSchema)]
pub struct KeyInfoOutput {
    pub keyname: String,
    pub keytype: KeyType,
}

use crate::{io::ValueEntry, parser::KeyType};
#[derive(Serialize, JsonSchema)]
pub enum Output {
    SpaceNames(Vec<String>),
    KeyNames(Vec<String>),
    GetValue(Vec<GetValueOutput>),
    SelectValue(Vec<SelectOutput>),
    SpaceTimeIdSet(SpaceTimeIdSet),
    Version(String),
    KeysInfo(Vec<KeyInfoOutput>),
    Success,
}
