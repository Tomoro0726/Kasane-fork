use super::{Error, StorageTrait, ValueEntry};
use sled::Db;
use std::path::PathBuf;

#[derive(Clone)]
pub struct Storage {
    pub inner: Db,
}

impl Storage {
    pub fn new(path: Option<PathBuf>) -> Result<Self, String> {
        let db_path = path.unwrap_or_else(|| std::env::current_dir().unwrap());
        let kv = sled::open(db_path).map_err(|e| e.to_string())?;
        kv.open_tree("space");
        kv.open_tree("key");
        kv.open_tree("value");
        Ok(Self { inner: kv })
    }
}

// StorageTrait の impl もここに集約可能
