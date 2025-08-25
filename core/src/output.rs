use kasane_logic::{
    id::{SpaceTimeId, coordinates::Point},
    set::SpaceTimeIdSet,
};

#[cfg(feature = "json_schema")]
use schemars::JsonSchema;
use serde::Serialize;
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Serialize)]
pub struct GetValueOutput {
    pub spacetimeid: SpaceTimeId,
    pub id_string: Option<String>,
    pub vertex: Option<[Point; 8]>,
    pub center: Option<Point>,
    pub value: ValueEntry,
}
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Serialize)]
pub struct SelectOutput {
    pub spacetimeid: SpaceTimeId,
    pub id_string: Option<String>,
    pub vertex: Option<[Point; 8]>,
    pub center: Option<Point>,
}
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Serialize)]
pub struct KeyInfoOutput {
    pub keyname: String,
    pub keytype: KeyType,
}

use crate::{io::ValueEntry, parser::KeyType};
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Serialize)]
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
