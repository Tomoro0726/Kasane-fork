use logic::set::SpaceTimeIdSet;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::parser::KeyType;
pub mod key;
pub mod space;
pub mod storage;

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

#[derive(Clone, PartialEq, Serialize, Deserialize, JsonSchema, Debug)]
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
