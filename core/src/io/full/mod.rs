use super::Error;
use lmdb::{Database, Environment};

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
