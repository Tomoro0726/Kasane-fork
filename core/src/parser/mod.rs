use logic::id::DimensionRange;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::io::ValueEntry;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]

pub struct AddSpace {
    pub spacename: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]

pub struct DeleteSpace {
    pub spacename: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]

pub struct AddKey {
    pub spacename: String,
    pub name: String,
    pub r#type: KeyType,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]

pub enum KeyType {
    INT,
    BOOLEAN,
    TEXT,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]

pub struct DeleteKey {
    pub spacename: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]

pub struct PutValue {
    pub spacename: String,
    pub keyname: String,
    pub select: Select,
    pub value: ValueEntry,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]

pub struct SetValue {
    pub spacename: String,
    pub keyname: String,
    pub select: Select,
    pub value: ValueEntry,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]

pub struct DeleteValue {
    pub spacename: String,
    pub keyname: String,
    pub select: Select,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub enum Select {
    Function(Function),
    Prefix(Prefix),
    SpaceTimeIdSet(Vec<SpaceTimeIdInput>),
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SpaceTimeIdInput {
    pub z: u16,
    pub f: DimensionRange<i64>,
    pub x: DimensionRange<u64>,
    pub y: DimensionRange<u64>,
    pub i: u32,
    pub t: DimensionRange<u32>,
}

// #[derive(Debug, Serialize, Deserialize, JsonSchema)]
// struct Line {
//     start_x: u32,
//     start_y: u32,
//     start_z: u32,
//     end_x: u32,
//     end_y: u32,
//     end_z: u32,
//     zoom: u32,
// }

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub enum Function {
    //Line(Line),
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]

pub enum Prefix {
    AND(Vec<Select>),
    OR(Vec<Select>),
    XOR(Vec<Select>),
    NOT(Box<Select>),
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Showkeys {
    pub spacename: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub enum Command {
    AddSpace(AddSpace),
    DeleteSpace(DeleteSpace),
    AddKey(AddKey),
    DeleteKey(DeleteKey),
    PutValue(PutValue),
    SetValue(SetValue),
    DeleteValue(DeleteValue),
    Showkeys(Showkeys),
    ShowSpaces,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Packet {
    pub user: String,
    pub commands: Vec<Command>,
    #[serde(rename = "$schema")]
    pub schema: String,
}
pub fn parser(packet_raw: &str) -> Result<Packet, serde_json::Error> {
    serde_json::from_str(packet_raw)
}
