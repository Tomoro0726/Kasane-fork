use kasane_logic::id::{DimensionRange, coordinates::Point};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::io::ValueEntry;

#[cfg(feature = "json_schema")]
use schemars::JsonSchema;

//共通型
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AllOrChoose<T> {
    Choose(T),
    All,
}

// ---------------------- Space管理 ----------------------
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateSpace {
    pub space_name: String,
}

#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DropSpace {
    pub space_name: String,
}

#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateKey {
    pub space_name: String,
    pub key_name: String,
    pub key_type: KeyType,
    pub key_mode: KeyMode,
}

#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum KeyMode {
    UniqueKey,
    MultiKey,
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
pub struct DropKey {
    pub space_name: String,
    pub key_name: String,
}

// ---------------------- Value管理 ----------------------
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InsertValue {
    pub space_name: String,
    pub key_name: String,
    pub range: Range,
    pub value: ValueEntry,
}

#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PatchValue {
    pub space_name: String,
    pub key_name: String,
    pub range: Range,
    pub value: ValueEntry,
}

#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateValue {
    pub space_name: String,
    pub key_name: String,
    pub range: Range,
    pub value: ValueEntry,
}

#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeleteValue {
    pub space_name: String,
    pub key_name: String,
    pub range: Range,
}

#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SelectValue {
    pub space_name: String,
    pub key_names: Vec<String>,
    pub range: Range,
    pub vertex: bool,
    pub center: bool,
    pub id_string: bool,
    pub id_pure: bool,
}

// ---------------------- Range & Function ----------------------
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Range {
    Function(Function),
    Prefix(Prefix),
    IdSet(Vec<IdInput>),
}

#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IdInput {
    pub z: u8,
    pub f: DimensionRange<i32>,
    pub x: DimensionRange<u32>,
    pub y: DimensionRange<u32>,
    pub i: u32,
    pub t: DimensionRange<u32>,
}

#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Line {
    pub start: Point,
    pub end: Point,
    pub zoom: u8,
}

#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Triangle {
    pub point1: Point,
    pub point2: Point,
    pub point3: Point,
    pub zoom: u8,
}

#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FilterValue {
    pub space_name: String,
    pub key_name: String,
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
    IsTrue,
    IsFalse,
    Equals(bool),
    NotEquals(bool),
}

#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FilterINT {
    Equal(i32),
    NotEqual(i32),
    GreaterThan(i32),
    GreaterEqual(i32),
    LessThan(i32),
    LessEqual(i32),
    Between(i32, i32),
    In(Vec<i32>),
    NotIn(Vec<i32>),
}

#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FilterTEXT {
    Equal(String),
    NotEqual(String),
    Contains(String),
    NotContains(String),
    StartsWith(String),
    EndsWith(String),
    CaseInsensitiveEqual(String),
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

// ---------------------- Key / Space情報 ----------------------
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShowKeys {
    pub space_name: String,
}

#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InfoKey {
    pub space_name: String,
    pub key_name: String,
}

#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InfoSpace {
    pub space_name: String,
}

#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShowValues {
    pub space_name: String,
    pub key_name: String,
}

// ---------------------- User管理 ----------------------
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateUser {
    pub user_name: String,
    pub password: String,
}

#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DropUser {
    pub user_name: String,
}

#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InfoUser {
    pub user_name: String,
}

// ---------------------- 権限管理 ----------------------
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GrantDatabase {
    pub user_name: String,
    pub command: AllOrChoose<Vec<CommandDatabase>>,
}

#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum CommandDatabase {
    CreateSpace,
    DropSpace,
    ShowSpaces,
    Version,
}

#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GrantSpacePrivilege {
    pub user_name: String,
    pub target_space: AllOrChoose<Vec<String>>,
    pub command: AllOrChoose<Vec<CommandSpace>>,
}

#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum CommandSpace {
    CreateKey,
    DropKey,
    InfoSpace,
    ShowKeys,
}

#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GrantKeyPrivilege {
    pub user_name: String,
    pub target_space: String,
    pub target_key: AllOrChoose<Vec<String>>,
    pub command: AllOrChoose<Vec<CommandKey>>,
}

#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CommandKey {
    InsertValue,
    PatchValue,
    UpdateValue,
    DropKey,
    SelectValue,
    InfoKey,
    ShowValues,
    FilterValue,
}

#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RevokeDatabase {
    pub user_name: String,
    pub command: AllOrChoose<Vec<CommandDatabase>>,
}
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RevokeSpacePrivilege {
    pub user_name: String,
    pub target_space: AllOrChoose<Vec<String>>,
    pub command: AllOrChoose<Vec<CommandSpace>>,
}
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RevokeKeyPrivilege {
    pub user_name: String,
    pub target_space: String,
    pub target_key: AllOrChoose<Vec<String>>,
    pub command: AllOrChoose<Vec<CommandKey>>,
}

// ---------------------- Packet & Command ----------------------
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Command {
    //データベース操作系
    CreateSpace(CreateSpace),
    DropSpace(DropSpace),
    InfoSpace(InfoSpace),
    ShowSpaces,
    Version,

    //Key操作系
    CreateKey(CreateKey),
    DropKey(DropKey),
    ShowKeys(ShowKeys),
    InfoKey(InfoKey),

    //Value操作系
    InsertValue(InsertValue),
    PatchValue(PatchValue),
    UpdateValue(UpdateValue),
    DeleteValue(DeleteValue),
    SelectValue(SelectValue),
    ShowValues(ShowValues),

    //ツール系
    //Transaction(Vec<Command>),

    //ユーザー操作系
    CreateUser(CreateUser),
    DropUser(DropUser),
    InfoUser(InfoUser),
    ShowUsers,
    // //権限付与系
    // GrantDatabase(GrantDatabase),
    // GrantSpacePrivilege(GrantSpacePrivilege),
    // GrantKeyPrivilege(GrantKeyPrivilege),

    // //権限取り上げ系
    // RevokeDatabase(RevokeDatabase),
    // RevokeSpacePrivilege(RevokeSpacePrivilege),
    // RevokeKeyPrivilege(RevokeKeyPrivilege),
}

#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize)]
pub struct Packet {
    pub command: Vec<Command>,
}

pub fn parser(value: &Value) -> Result<Packet, serde_json::Error> {
    serde_json::from_value(value.clone())
}
