use logic::set::SpaceTimeIdSet;
use schemars::JsonSchema;
use serde::Serialize;

use crate::io::output::IoOutput;
#[derive(Serialize, JsonSchema)]
pub enum Output {
    IoResult(IoOutput),
    SpaceTimeIdSet(SpaceTimeIdSet),
}
