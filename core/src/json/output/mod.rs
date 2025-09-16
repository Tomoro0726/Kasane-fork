use std::collections::{HashMap, HashSet};

use kasane_logic::id::{SpaceTimeId, coordinates::Point};
use serde::Serialize;

use crate::{
    io::ValueEntry,
    json::input::{CommandDatabase, CommandKey, CommandSpace},
};

#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Serialize)]
pub struct ShowSpaces {
    pub spacenames: Vec<String>,
}

#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Serialize)]
pub struct Version {
    pub version: String,
}

#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Serialize)]
pub struct InfoSpace {
    pub spacename: String,
    pub keynames: Vec<InfoKey>,
}

#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Serialize)]
pub struct InfoKey {
    pub keyname: String,
    pub keytype: String,
}

#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Serialize)]
pub struct Showkeys {
    pub keynames: Vec<String>,
}

#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Serialize)]
pub struct SelectValue {
    values: Vec<Value>,
}

#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Serialize)]
pub struct Value {
    id: SpaceTimeId,
    center: Option<Point>,
    vertex: Option<[Point; 8]>,
    id_string: Option<String>,
    value: HashMap<String, ValueEntry>,
}

#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Serialize)]
pub struct InfoUser {
    user_name: String,
    database_command: Vec<CommandDatabase>,
    space_command: Vec<InfoUserSpace>,
    key_commnad: Vec<InfoUserKey>,
}

#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Serialize)]
pub struct InfoUserSpace {
    space_name: String,
    space_commnad: Vec<CommandSpace>,
}

#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Serialize)]
pub struct InfoUserKey {
    space_name: String,
    key_name: String,
    space_commnad: Vec<CommandKey>,
}

#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Output {
    //CreateSpace,DropSpace,CreateKey,DropKey,InsertValue,UpdateValue,DeleteValue,CreateUser,DropUser,GrantDatabase,GrantSpacePrivilege,GrantKeyPrivilege,GrantToolPrivilege,RevokeDatabase,RevokeSpacePrivilege,RevokeKeyPrivilege,RevokeToolPrivilege
    Success,

    //データベース操作系
    InfoSpace(InfoSpace),
    ShowSpaces(ShowSpaces),
    Version(Version),

    //Key操作系
    Showkeys(Showkeys),
    InfoKey(InfoKey),

    //Value操作系
    SelectValue(SelectValue),

    //ユーザー操作系
    InfoUser(InfoUser),
}
