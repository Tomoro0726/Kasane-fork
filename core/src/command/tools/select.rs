use logic::set::SpaceTimeIdSet;

use crate::parser::Function::Line;
use crate::parser::Prefix::{AND, NOT, OR};
use crate::parser::Select;

pub fn select(v: Select) -> SpaceTimeIdSet {
    match v {
        Select::Function(function) => match function {
            Line(line) => {
                todo!()
            }
        },
        Select::Prefix(prefix) => match prefix {
            AND(and) => {
                let mut is_first = true;
                let mut result = SpaceTimeIdSet::new();
                for ele in and {
                    if is_first {
                        result = result | (select(ele));
                        is_first = false
                    } else {
                        result = result & select(ele)
                    }
                }
                result
            }
            OR(or) => {
                let mut result = SpaceTimeIdSet::new();
                for ele in or {
                    result = result | select(ele)
                }
                result
            }
            NOT(not) => !select(*not),
        },
        Select::SpaceTimeIdSet(set) => {
            return set;
        }
    }
}
