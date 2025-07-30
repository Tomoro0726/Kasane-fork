use std::collections::HashMap;

use logic::set::SpaceTimeIdSet;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]

struct AddSpace {
    spacename: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]

struct DeleteSpace {
    spacename: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]

struct AddKeys {
    spacename: String,
    keys: Vec<AddKeyInfo>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]

struct AddKeyInfo {
    name: String,
    r#type: KeyType,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]

enum KeyType {
    Int,
    Boolean,
    Text,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]

struct DeleteKeys {
    spacename: String,
    keys: Vec<DeleteKeyInfo>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]

struct DeleteKeyInfo {
    name: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]

struct PutValue {
    select: Select,
    keys: Vec<SelectKeyInfo>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]

struct SetValue {
    select: Select,
    keys: Vec<SelectKeyInfo>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]

struct DeleteValue {
    select: Select,
    keys: Vec<SelectKeyInfo>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]

struct SelectKeyInfo {
    name: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]

enum Select {
    Function(Function),
    Prefix(Prefix),
    SpaceTimeId(SpaceTimeIdSet),
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]

struct Line {
    start_x: u32,
    start_y: u32,
    start_z: u32,
    end_x: u32,
    end_y: u32,
    end_z: u32,
    zoom: u32,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]

enum Function {
    Line(Line),
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]

enum Prefix {
    AND(Vec<Select>),
    OR(Vec<Select>),
    NOT(Vec<Select>),
}
#[derive(Debug, Serialize, Deserialize, JsonSchema)]

struct Transaction {
    action: TransactionAction,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]

enum TransactionAction {
    Start,
    Commit,
    Rollback,
}
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub enum Command {
    AddSpace(AddSpace),
    DeleteSpace(DeleteSpace),
    AddKeys(AddKeys),
    DeleteKeys(DeleteKeys),
    PutValue(PutValue),
    SetValue(SetValue),
    DeleteValue(DeleteValue),
    Transaction(Transaction),
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Packet {
    pub user: String,
    pub commands: Vec<Command>,
    // #[serde(flatten)]
    // extra: HashMap<String, serde_json::Value>,
}

pub fn parser(packet_raw: &str) -> Result<Packet, serde_json::Error> {
    serde_json::from_str(packet_raw)
}
