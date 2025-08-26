use super::{Error, StorageTrait, ValueEntry};
use std::{cell::RefCell, collections::BTreeMap};

#[derive(Clone)]
pub struct Storage {
    pub inner: RefCell<BTreeMap<String, String>>,
}

impl Storage {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            inner: RefCell::new(BTreeMap::new()),
        })
    }
}

// StorageTrait の impl もここに集約可能
impl StorageTrait for Storage {
    fn transaction<F>(&self, cmds: Vec<F>) -> Result<Vec<crate::json::output::Output>, Error>
    where
        F: Fn(&Self) -> Result<crate::json::output::Output, Error>,
    {
        todo!()
    }

    fn show_spaces(&self) -> Result<crate::json::output::Output, Error> {
        todo!()
    }

    fn add_space(&self, spacename: &str) -> Result<crate::json::output::Output, Error> {
        self.inner.borrow_mut().insert(key, value)
    }

    fn delete_space(&self, spacename: &str) -> Result<crate::json::output::Output, Error> {
        todo!()
    }

    fn show_keys(&self, spacename: &str) -> Result<crate::json::output::Output, Error> {
        todo!()
    }

    fn info_space(&self, spacename: &str) -> Result<crate::json::output::Output, Error> {
        todo!()
    }

    fn add_key(
        &self,
        spacename: &str,
        keyname: &str,
        keytype: crate::json::input::KeyType,
    ) -> Result<crate::json::output::Output, Error> {
        todo!()
    }

    fn delete_key(
        &self,
        spacename: &str,
        keyname: &str,
    ) -> Result<crate::json::output::Output, Error> {
        todo!()
    }

    fn info_key(
        &self,
        spacename: &str,
        keyname: &str,
    ) -> Result<crate::json::output::Output, Error> {
        todo!()
    }

    fn filter_value(
        &self,
        spacename: &str,
        keyname: &str,
        filter: crate::json::input::FilterType,
    ) -> Result<crate::json::output::Output, Error> {
        todo!()
    }

    fn get_value(
        &self,
        spacename: &str,
        keyname: &str,
        set: kasane_logic::set::SpaceTimeIdSet,
    ) -> Result<crate::json::output::Output, Error> {
        todo!()
    }

    fn set_value(
        &self,
        spacename: &str,
        keyname: &str,
        value: ValueEntry,
        set: kasane_logic::set::SpaceTimeIdSet,
    ) -> Result<crate::json::output::Output, Error> {
        todo!()
    }

    fn put_value(
        &self,
        spacename: &str,
        keyname: &str,
        value: ValueEntry,
        set: kasane_logic::set::SpaceTimeIdSet,
    ) -> Result<crate::json::output::Output, Error> {
        todo!()
    }

    fn delete_value(
        &self,
        spacename: &str,
        keyname: &str,
        set: kasane_logic::set::SpaceTimeIdSet,
    ) -> Result<crate::json::output::Output, Error> {
        todo!()
    }
}
