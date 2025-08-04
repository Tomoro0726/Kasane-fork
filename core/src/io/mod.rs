use logic::set::SpaceTimeIdSet;
pub mod error;
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
    value: Vec<Value>,
}

struct Value {
    value: ValueEntry,
    set: SpaceTimeIdSet,
}

enum ValueEntry {
    INT(i64),
    TEXT(String),
    BOOLEAN(bool),
}

impl Storage {
    pub fn new() -> Result<Self, String> {
        return Ok(Storage { space: Vec::new() });
    }
}
