use std::collections::{HashMap, HashSet};

use logic::set::SpaceTimeIdSet;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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
    Int,
    Boolean,
    Text,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]

pub struct DeleteKeys {
    spacename: String,
    keys: Vec<DeleteKeyInfo>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]

pub struct DeleteKeyInfo {
    name: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]

pub struct PutValue {
    select: Select,
    keys: Vec<SelectKeyInfo>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]

pub struct SetValue {
    select: Select,
    keys: Vec<SelectKeyInfo>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]

pub struct DeleteValue {
    select: Select,
    keys: Vec<SelectKeyInfo>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]

pub struct SelectKeyInfo {
    name: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub enum Select {
    Function(Function),
    Prefix(Prefix),
    SpaceTimeIdSet(SpaceTimeIdSet),
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

pub enum Function {
    Line(Line),
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]

pub enum Prefix {
    //XORの追加
    AND(Vec<Select>),
    OR(Vec<Select>),
    XOR(Vec<Select>),
    NOT(Box<Select>),
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
struct Showkeys {
    spacename: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct AddUser {
    username: String,
    password: String,
    //permisson: HashSet<Command>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct DeleteUser {
    username: String,
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
    Showkeys(Showkeys),
    AddUser(AddUser),
    DeleteUser(DeleteUser),
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
