use std::path::PathBuf;

use sled::Db;

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
            return Ok(Storage { kv });
        }
    }
}
