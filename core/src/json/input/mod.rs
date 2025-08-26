use std::clone;

use kasane_logic::id::{DimensionRange, coordinates::Point};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::io::ValueEntry;

#[cfg(feature = "json_schema")]
use schemars::JsonSchema;

#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AddSpace {
    pub spacename: String,
}
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]

pub struct DeleteSpace {
    pub spacename: String,
}
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]

pub struct AddKey {
    pub spacename: String,
    pub keyname: String,
    pub keytype: KeyType,
}
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum KeyType {
    INT,
    BOOLEAN,
    TEXT,
    FLOAT,
}
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]

pub struct DeleteKey {
    pub spacename: String,
    pub keyname: String,
}
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]

pub struct PutValue {
    pub spacename: String,
    pub keyname: String,
    pub range: Range,
    pub value: ValueEntry,
}
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]

pub struct SetValue {
    pub spacename: String,
    pub keyname: String,
    pub range: Range,
    pub value: ValueEntry,
}
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]

pub struct DeleteValue {
    pub spacename: String,
    pub keyname: String,
    pub range: Range,
}
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetValue {
    pub spacename: String,
    pub keyname: String,
    pub range: Range,
    pub vertex: bool,
    pub center: bool,
    pub id_string: bool,
    pub id_pure: bool,
}
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Range {
    Function(Function),
    Prefix(Prefix),
    SpaceTimeIdSet(Vec<SpaceTimeIdInput>),
}
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpaceTimeIdInput {
    pub z: u16,
    pub f: DimensionRange<i64>,
    pub x: DimensionRange<u64>,
    pub y: DimensionRange<u64>,
    pub i: u32,
    pub t: DimensionRange<u32>,
}
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Line {
    pub start: Point,
    pub end: Point,
    pub zoom: u16,
}
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Triangle {
    pub point1: Point,
    pub point2: Point,
    pub point3: Point,
    pub zoom: u16,
}
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FilterValue {
    pub spacename: String,
    pub keyname: String,
    pub filter: FilterType,
}
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FilterType {
    HasValue,
    FilterBOOLEAN(FilterBOOLEAN),
    FilterINT(FilterINT),
    FilterTEXT(FilterTEXT),
}
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FilterBOOLEAN {
    IsTrue,          // 真である
    IsFalse,         // 偽である
    Equals(bool),    // 指定の真偽値と等しいか
    NotEquals(bool), // 指定の真偽値と等しくないか
}
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FilterINT {
    Equal(i32),        // 等しい
    NotEqual(i32),     // 等しくない
    GreaterThan(i32),  // より大きい
    GreaterEqual(i32), // 以上
    LessThan(i32),     // より小さい
    LessEqual(i32),    // 以下
    Between(i32, i32), // 範囲内（inclusive）
    In(Vec<i32>),      // 指定した複数の値のいずれか
    NotIn(Vec<i32>),   // 指定した複数の値以外
}
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FilterTEXT {
    Equal(String),                // 完全一致
    NotEqual(String),             // 完全不一致
    Contains(String),             // 部分一致
    NotContains(String),          // 部分不一致
    StartsWith(String),           // 前方一致
    EndsWith(String),             // 後方一致
    CaseInsensitiveEqual(String), // 大文字小文字無視の完全一致
}
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Function {
    Line(Line),
    Triangle(Triangle),
    FilterValue(FilterValue),
}
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]

pub enum Prefix {
    AND(Vec<Range>),
    OR(Vec<Range>),
    XOR(Vec<Range>),
    NOT(Vec<Range>),
}
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Keys {
    pub spacename: String,
}
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Select {
    pub range: Range,
    pub vertex: bool,
    pub center: bool,
    pub id_string: bool,
    pub id_pure: bool,
}
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KeysInfo {
    pub spacename: String,
}

#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Command {
    AddSpace(AddSpace),
    DeleteSpace(DeleteSpace),
    AddKey(AddKey),
    DeleteKey(DeleteKey),
    PutValue(PutValue),
    SetValue(SetValue),
    DeleteValue(DeleteValue),
    GetValue(GetValue),
    Keys(Keys),
    Spaces,
    Select(Select),
    Version,
    KeysInfo(KeysInfo),
    Transaction(Vec<Command>),
}

#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize)]
pub struct Packet {
    pub command: Vec<Command>,
}

pub fn parser(value: &Value) -> Result<Packet, serde_json::Error> {
    serde_json::from_value(value.clone())
}
