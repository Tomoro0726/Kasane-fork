use std::env;
use std::path::PathBuf;

use super::{Error, StorageTrait, ValueEntry};
use crate::io::tools::key_bytes::key_bytes;
use crate::json::input::{FilterType, KeyType};
use crate::json::output::Output;
use kasane_logic::set::SpaceTimeIdSet;
use lmdb::{Cursor, Error as LmdbError};
use lmdb::{Database, DatabaseFlags, Environment, Transaction, WriteFlags};
use uuid::Uuid;
pub mod addspace;

pub struct Storage {
    pub space: Database,
    pub key: Database,
    pub value: Database,
    pub env: Environment,
}

impl From<lmdb::Error> for Error {
    fn from(e: lmdb::Error) -> Self {
        match e {
            lmdb::Error::MapFull => Error::LmdbMapFull {
                attempted_size: 0, // 必要に応じて Environment から取得して渡す
                location: "unknown",
            },
            lmdb::Error::NotFound => Error::LmdbDbNotFound {
                db_name: "unknown".to_string(),
                location: "unknown",
            },
            _ => Error::LmdbError {
                message: format!("{}", e),
                location: "unknown",
            },
        }
    }
}
