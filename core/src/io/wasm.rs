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
