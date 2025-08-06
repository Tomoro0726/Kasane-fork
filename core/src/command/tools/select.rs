use logic::id::SpaceTimeId;
use logic::set::SpaceTimeIdSet;

use crate::command::error::CommandError;
use crate::error::Error;
use crate::parser::Prefix::{AND, NOT, OR, XOR};
use crate::parser::Select;

pub fn select(v: Select) -> Result<SpaceTimeIdSet, Error> {
    match v {
        Select::Prefix(prefix) => match prefix {
            AND(and) => {
                let mut is_first = true;
                let mut result = SpaceTimeIdSet::new();
                for ele in and {
                    let set = select(ele)?;
                    if is_first {
                        result = set;
                        is_first = false;
                    } else {
                        result = result & set;
                    }
                }
                Ok(result)
            }

            OR(or) => {
                let mut result = SpaceTimeIdSet::new();
                for ele in or {
                    let set = select(ele)?;
                    result = result | set;
                }
                Ok(result)
            }

            NOT(not) => {
                let set = select(*not)?;
                Ok(!set)
            }

            XOR(xor) => {
                let mut is_first = true;
                let mut result = SpaceTimeIdSet::new();
                for ele in xor {
                    let set = select(ele)?;
                    if is_first {
                        result = set;
                        is_first = false;
                    } else {
                        result = result ^ set;
                    }
                }
                Ok(result)
            }
        },

        Select::SpaceTimeIdSet(ids) => {
            let mut set = SpaceTimeIdSet::new();
            for id in ids {
                let new_id = SpaceTimeId::new(id.z, id.f, id.x, id.y, id.i, id.t)
                    .map_err(|e| Error::CommandError(CommandError::ParseError(e)))?;
                set.insert(new_id);
            }
            Ok(set)
        }
    }
}
