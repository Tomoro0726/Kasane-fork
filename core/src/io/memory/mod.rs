use kasane_logic::set::SpaceTimeIdSet;
pub mod key;
pub mod space;
pub mod storage;

#[cfg(feature = "json_schema")]
use schemars::JsonSchema;

use crate::{io::ValueEntry, json::input::KeyType};

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

impl Storage {
    pub fn new() -> Result<Self, String> {
        return Ok(Storage { space: Vec::new() });
    }
}
