use std::path::PathBuf;

use sled::Db;

use crate::error::Error;
pub mod space;
pub mod storage;

#[derive(Clone)]
pub struct Storage {
    kv: Db,
}

impl Storage {
    pub fn new(path: Option<PathBuf>) -> Result<Self, String> {
        {
            use std::env;
            let db_path = path
                .unwrap_or_else(|| env::current_dir().expect("Failed to get current directory"));
            let kv = sled::open(db_path).map_err(|e| e.to_string())?;
            kv.open_tree("manage");
            kv.open_tree("data");
            return Ok(Storage { kv });
        }
    }
}

impl From<sled::Error> for Error {
    fn from(err: sled::Error) -> Self {
        Error::StorageError {
            source: err,
            location: "unknown", // 適宜場所情報をセットする
        }
    }
}
