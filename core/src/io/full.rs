use std::{env, path::PathBuf};

use crate::{io::StorageTrait, json::output::Output};
use lmdb::{DatabaseFlags, Error as LmdbError};

use super::Error;
use lmdb::{Database, Environment, Transaction};
use uuid::Uuid;

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

impl Storage {
    pub fn new(path: Option<PathBuf>) -> Result<Self, Error> {
        // LMDB 環境を作成
        let env = Environment::new()
            .set_max_dbs(10) // 名前付きDBの上限
            .set_map_size(1024 * 1024 * 1024) // 1GB
            .open(&path.unwrap_or(env::current_dir().unwrap()))?;

        // データベースを開く（なければ作成）
        let space = env.create_db(Some("space"), DatabaseFlags::empty())?;

        let key = env.create_db(Some("key"), DatabaseFlags::empty())?;
        let value = env.create_db(Some("value"), DatabaseFlags::empty())?;

        Ok(Self {
            space,
            key,
            value,
            env,
        })
    }
}

impl StorageTrait for Storage {
    fn create_space(&self, spacename: &str) -> Result<Output, Error> {
        let space_id: [u8; 16] = *Uuid::new_v4().as_bytes();
        let space_bytes = spacename.as_bytes();

        let mut txn = self.env.begin_rw_txn()?;
        txn.put(
            self.space,
            &space_bytes,
            &space_id,
            lmdb::WriteFlags::empty(),
        )
        //既に同じ名前のSpaceが存在する場合にはエラーを返す
        .map_err(|e| match e {
            LmdbError::KeyExist => Error::SpaceAlreadyExists {
                space_name: spacename.to_owned(),
            },
            _ => Error::from(e),
        })?;
        txn.commit()?;
        Ok(Output::Success)
    }

    fn drop_space(&self, spacename: &str) -> Result<crate::json::output::Output, Error> {
        todo!()
    }

    fn info_space(&self, spacename: &str) -> Result<crate::json::output::Output, Error> {
        todo!()
    }

    fn show_spaces(&self) -> Result<crate::json::output::Output, Error> {
        todo!()
    }

    fn version() -> Result<crate::json::output::Output, Error> {
        todo!()
    }

    fn create_key(
        &self,
        spacename: &str,
        keyname: &str,
        keytype: crate::json::input::KeyType,
        keymode: crate::json::input::KeyMode,
    ) -> Result<crate::json::output::Output, Error> {
        todo!()
    }

    fn drop_key(
        &self,
        spacename: &str,
        keyname: &str,
    ) -> Result<crate::json::output::Output, Error> {
        todo!()
    }

    fn show_keys(&self, spacename: &str) -> Result<crate::json::output::Output, Error> {
        todo!()
    }

    fn info_key(
        &self,
        spacename: &str,
        keyname: &str,
    ) -> Result<crate::json::output::Output, Error> {
        todo!()
    }

    fn insert_value(
        &self,
        spacename: &str,
        keyname: &str,
        ids: std::collections::HashSet<kasane_logic::id::SpaceTimeId>,
        value: super::ValueEntry,
    ) -> Result<crate::json::output::Output, Error> {
        todo!()
    }

    fn patch_value(
        &self,
        spacename: &str,
        keyname: &str,
        ids: std::collections::HashSet<kasane_logic::id::SpaceTimeId>,
        value: super::ValueEntry,
    ) -> Result<crate::json::output::Output, Error> {
        todo!()
    }

    fn update_value(
        &self,
        spacename: &str,
        keyname: &str,
        ids: std::collections::HashSet<kasane_logic::id::SpaceTimeId>,
        value: super::ValueEntry,
    ) -> Result<crate::json::output::Output, Error> {
        todo!()
    }

    fn delete_value(
        &self,
        spacename: &str,
        keyname: &str,
        ids: std::collections::HashSet<kasane_logic::id::SpaceTimeId>,
    ) -> Result<crate::json::output::Output, Error> {
        todo!()
    }

    fn select_value(
        &self,
        spacename: &str,
        keyname: &str,
        id: std::collections::HashSet<kasane_logic::id::SpaceTimeId>,
    ) -> Result<crate::json::output::Output, Error> {
        todo!()
    }

    fn show_values(
        &self,
        spacename: &str,
        keyname: &str,
    ) -> Result<crate::json::output::Output, Error> {
        todo!()
    }

    fn create_user(
        &self,
        username: &str,
        password: &str,
    ) -> Result<crate::json::output::Output, Error> {
        todo!()
    }

    fn drop_user(&self, username: &str) -> Result<crate::json::output::Output, Error> {
        todo!()
    }

    fn info_user(&self, username: &str) -> Result<crate::json::output::Output, Error> {
        todo!()
    }

    fn show_users(&self) -> Result<crate::json::output::Output, Error> {
        todo!()
    }
}
