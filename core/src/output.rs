use logic::set::SpaceTimeIdSet;
use schemars::JsonSchema;
use serde::Serialize;

use crate::io::ValueEntry;
#[derive(Serialize, JsonSchema)]
pub enum Output {
    SpaceNames(Vec<String>),
    KeyNames(Vec<String>),
    GetValue(Vec<(SpaceTimeIdSet, ValueEntry)>),
    SpaceTimeIdSet(SpaceTimeIdSet),
    Success,
}
