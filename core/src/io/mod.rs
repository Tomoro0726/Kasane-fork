use crate::parser::KeyType;
use kasane_logic::set::SpaceTimeIdSet;
use serde::{Deserialize, Serialize};
pub mod key;
pub mod space;
pub mod storage;

#[cfg(feature = "json_schema")]
use schemars::JsonSchema;

#[derive(Debug)]
pub struct Storage {
    space: Vec<Space>,
}

#[derive(Debug)]
pub struct Space {
    name: String,
    key: Vec<Key>,
}

#[derive(Debug)]
pub struct Key {
    pub name: String,
    pub r#type: KeyType,
    pub value: Vec<Value>,
}

#[derive(Debug)]
pub struct Value {
    value: ValueEntry,
    set: SpaceTimeIdSet,
}

#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum ValueEntry {
    INT(i32),
    TEXT(String),
    BOOLEAN(bool),
    FLOAT(f32),
}

impl Storage {
    pub fn new() -> Result<Self, String> {
        return Ok(Storage { space: Vec::new() });
    }
}
