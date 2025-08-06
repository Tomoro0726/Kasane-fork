use logic::set::SpaceTimeIdSet;

use crate::io::output::IoOutput;

pub enum Output {
    IoResult(IoOutput),
    SpaceTimeIdSet(SpaceTimeIdSet),
}
