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
    pub keyname: String,
    pub r#type: KeyType,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, PartialEq)]
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
    pub select: Range,
    pub value: ValueEntry,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]

pub struct SetValue {
    pub spacename: String,
    pub keyname: String,
    pub select: Range,
    pub value: ValueEntry,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]

pub struct DeleteValue {
    pub spacename: String,
    pub keyname: String,
    pub select: Range,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GetValue {
    pub spacename: String,
    pub keyname: String,
    pub select: Range,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub enum Range {
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
pub struct HasValue {
    pub spacename: String,
    pub keyname: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FilterValue {
    pub spacename: String,
    pub keyname: String,
    pub filter: FilterType,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub enum FilterType {
    FilterBOOLEAN(FilterBOOLEAN),
    FilterINT(FilterINT),
    FilterTEXT(FilterTEXT),
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub enum FilterBOOLEAN {
    IsTrue,          // 真である
    IsFalse,         // 偽である
    Equals(bool),    // 指定の真偽値と等しいか
    NotEquals(bool), // 指定の真偽値と等しくないか
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub enum FilterINT {
    Equal(i64),        // 等しい
    NotEqual(i64),     // 等しくない
    GreaterThan(i64),  // より大きい
    GreaterEqual(i64), // 以上
    LessThan(i64),     // より小さい
    LessEqual(i64),    // 以下
    Between(i64, i64), // 範囲内（inclusive）
    In(Vec<i64>),      // 指定した複数の値のいずれか
    NotIn(Vec<i64>),   // 指定した複数の値以外
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub enum FilterTEXT {
    Equal(String),                // 完全一致
    NotEqual(String),             // 完全不一致
    Contains(String),             // 部分一致
    NotContains(String),          // 部分不一致
    StartsWith(String),           // 前方一致
    EndsWith(String),             // 後方一致
    CaseInsensitiveEqual(String), // 大文字小文字無視の完全一致
    Regex(String),                // 正規表現マッチ
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub enum Function {
    //Line(Line),
    FilterValue(FilterValue),
    HasValue(HasValue),
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]

pub enum Prefix {
    AND(Vec<Range>),
    OR(Vec<Range>),
    XOR(Vec<Range>),
    NOT(Vec<Range>),
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct keys {
    pub spacename: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct spaces {}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub enum Command {
    AddSpace(AddSpace),
    DeleteSpace(DeleteSpace),
    AddKey(AddKey),
    DeleteKey(DeleteKey),
    PutValue(PutValue),
    SetValue(SetValue),
    DeleteValue(DeleteValue),
    GetValue(GetValue),
    Keys(keys),
    Spaces(spaces),
}

pub fn parser(packet_raw: &str) -> Result<Command, serde_json::Error> {
    serde_json::from_str(packet_raw)
}
