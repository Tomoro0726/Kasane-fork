use std::collections::HashSet;

use crate::{
    error::Error,
    json::{
        input::{KeyMode, KeyType},
        output::Output,
    },
};
use kasane_logic::id::SpaceTimeId;
use serde::{Deserialize, Serialize};
pub mod full;
pub mod tools;

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum ValueEntry {
    TEXT(String),
    BOOLEAN(bool),
    INT(i32),
    FLOAT(f32),
}

// StorageTrait は共通
pub trait StorageTrait {
    //データベース操作系
    fn create_space(&self, spacename: &str) -> Result<Output, Error>;
    fn drop_space(&self, spacename: &str) -> Result<Output, Error>;
    fn info_space(&self, spacename: &str) -> Result<Output, Error>;
    fn show_spaces(&self) -> Result<Output, Error>;

    //key操作系
    fn create_key(
        &self,
        spacename: &str,
        keyname: &str,
        keytype: KeyType,
        keymode: KeyMode,
    ) -> Result<Output, Error>;
    fn drop_key(&self, spacename: &str, keyname: &str) -> Result<Output, Error>;
    fn show_keys(&self, spacename: &str) -> Result<Output, Error>;
    fn info_key(&self, spacename: &str, keyname: &str) -> Result<Output, Error>;

    //Value操作系

    fn insert_value(
        &self,
        spacename: &str,
        keyname: &str,
        ids: HashSet<SpaceTimeId>,
        value: ValueEntry,
    ) -> Result<Output, Error>;
    fn patch_value(
        &self,
        spacename: &str,
        keyname: &str,
        ids: HashSet<SpaceTimeId>,
        value: ValueEntry,
    ) -> Result<Output, Error>;
    fn update_value(
        &self,
        spacename: &str,
        keyname: &str,
        ids: HashSet<SpaceTimeId>,
        value: ValueEntry,
    ) -> Result<Output, Error>;
    fn delete_value(
        &self,
        spacename: &str,
        keyname: &str,
        ids: HashSet<SpaceTimeId>,
    ) -> Result<Output, Error>;
    fn select_value(
        &self,
        spacename: &str,
        keyname: &str,
        id: HashSet<SpaceTimeId>,
    ) -> Result<Output, Error>;
    fn show_values(&self, spacename: &str, keyname: &str) -> Result<Output, Error>;

    //ユーザー操作系
    fn create_user(&self, username: &str, password: &str) -> Result<Output, Error>;
    fn drop_user(&self, username: &str) -> Result<Output, Error>;
    fn info_user(&self, username: &str) -> Result<Output, Error>;
    fn show_users(&self) -> Result<Output, Error>;
}
