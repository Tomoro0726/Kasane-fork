use serde::{Deserialize, Serialize};

#[cfg(feature = "wasm")]
pub mod memory;

#[cfg(feature = "default")]
pub mod kv;

#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum ValueEntry {
    INT(i32),
    TEXT(String),
    BOOLEAN(bool),
    FLOAT(f32),
}
