use crate::{
    error::Error,
    json::{input::FilterType, output::Output},
};
use kasane_logic::set::SpaceTimeIdSet;
use serde::{Deserialize, Serialize};

#[cfg(feature = "full")]
mod full;
#[cfg(feature = "wasm")]
mod wasm;

#[cfg(feature = "full")]
pub use full::*;
#[cfg(feature = "wasm")]
pub use wasm::*;

// ValueEntry は共通
#[cfg_attr(feature = "json_schema", derive(schemars::JsonSchema))]
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum ValueEntry {
    INT(i32),
    TEXT(String),
    BOOLEAN(bool),
    FLOAT(f32),
}

// StorageTrait は共通
pub trait StorageTrait {
    fn transaction<F>(&mut self, cmds: Vec<F>) -> Result<Vec<Output>, Error>
    where
        F: Fn(&mut Self) -> Result<Output, Error>;

    fn show_spaces(&mut self) -> Result<Output, Error>;
    fn add_space(&mut self, spacename: &str) -> Result<Output, Error>;
    fn delete_space(&mut self, spacename: &str) -> Result<Output, Error>;
    fn show_keys(&self, spacename: &str) -> Result<Vec<String>, Error>;
    fn info_space(&mut self, spacename: &str) -> Result<Output, Error>;
    fn add_key(&mut self, spacename: &str, keyname: &str) -> Result<Output, Error>;
    fn delete_key(&mut self, spacename: &str, keyname: &str) -> Result<Output, Error>;
    fn info_key(&mut self, spacename: &str, keyname: &str) -> Result<Output, Error>;
    fn filter_value(
        &mut self,
        spacename: &str,
        keyname: &str,
        filter: FilterType,
    ) -> Result<Output, Error>;
    fn get_value(
        &mut self,
        spacename: &str,
        keyname: &str,
        set: SpaceTimeIdSet,
    ) -> Result<Output, Error>;
    fn set_value(
        &mut self,
        spacename: &str,
        keyname: &str,
        value: ValueEntry,
        set: SpaceTimeIdSet,
    ) -> Result<Output, Error>;
    fn put_value(
        &mut self,
        spacename: &str,
        keyname: &str,
        value: ValueEntry,
        set: SpaceTimeIdSet,
    ) -> Result<Output, Error>;
    fn delete_value(
        &mut self,
        spacename: &str,
        keyname: &str,
        set: SpaceTimeIdSet,
    ) -> Result<Output, Error>;
}
