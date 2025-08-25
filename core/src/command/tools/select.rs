use kasane_logic::id::SpaceTimeId;
use kasane_logic::set::SpaceTimeIdSet;

use crate::command::line::line;
use crate::command::triangle::triangle;
use crate::error::Error;
use crate::io::Storage;
use crate::json::input::Prefix::{AND, NOT, OR, XOR};
use crate::json::input::{Function, Range};

pub fn select(s: &mut Storage, v: Range) -> Result<SpaceTimeIdSet, Error> {
    match v {
        Range::Prefix(prefix) => match prefix {
            AND(and) => {
                let mut is_first = true;
                let mut result = SpaceTimeIdSet::new();
                for ele in and {
                    let set = select(s, ele)?;
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
                    let set = select(s, ele)?;
                    result = result | set;
                }
                Ok(result)
            }

            NOT(not) => {
                let mut result = SpaceTimeIdSet::new();
                for ele in not {
                    let set = select(s, ele)?;
                    result = result | set;
                }
                Ok(!result)
            }

            XOR(xor) => {
                let mut is_first = true;
                let mut result = SpaceTimeIdSet::new();
                for ele in xor {
                    let set = select(s, ele)?;
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
        Range::SpaceTimeIdSet(ids) => {
            let mut set = SpaceTimeIdSet::new();
            for id in ids {
                let new_id = SpaceTimeId::new(id.z, id.f, id.x, id.y, id.i, id.t).map_err(|e| {
                    Error::ParseError {
                        message: e,
                        location: "command::tools::select::select",
                    }
                })?;
                set.insert(new_id);
            }
            Ok(set)
        }
        Range::Function(function) => match function {
            Function::HasValue(v) => {
                let space = s.get_space(&v.spacename)?;
                let key = space.get_key(&v.keyname)?;
                return Ok(key.has_value());
            }
            Function::Line(v) => Ok(line(v)),
            Function::Triangle(v) => Ok(triangle(v)),
            Function::FilterValue(v) => {
                let space = s.get_space(&v.spacename)?;
                let key = space.get_key(&v.keyname)?;
                key.filter_value(v.filter)
            }
        },
    }
}
