use logic::set::SpaceTimeIdSet;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::parser::KeyType;
pub mod key;
pub mod space;
pub mod storage;

pub struct Storage {
    space: Vec<Space>,
}

pub struct Space {
    name: String,
    key: Vec<Key>,
}

pub struct Key {
    name: String,
    r#type: KeyType,
    value: Vec<Value>,
}

pub struct Value {
    value: ValueEntry,
    set: SpaceTimeIdSet,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Debug)]
pub enum ValueEntry {
    INT(i64),
    TEXT(String),
    BOOLEAN(bool),
}

impl Storage {
    pub fn new() -> Result<Self, String> {
        return Ok(Storage { space: Vec::new() });
    }
}
