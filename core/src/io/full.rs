use std::env;
use std::fmt::format;
use std::path::PathBuf;

use super::{Error, StorageTrait, ValueEntry};
use crate::io::tools::keytype_id::keytype_id;
use crate::json::input::{FilterType, KeyType};
use crate::json::output::Output;
use kasane_logic::set::SpaceTimeIdSet;
use lmdb::{Cursor, Error as LmdbError};
use lmdb::{Database, DatabaseFlags, Environment, Transaction, WriteFlags};
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
    fn show_spaces(&self) -> Result<Output, Error> {
        let txn = self.env.begin_ro_txn()?;
        let mut cursor = txn.open_ro_cursor(self.space)?;

        let mut spaces = Vec::new();

        for result in cursor.iter_start() {
            let (key, _value) = result;
            // key が &[u8] なので String に変換
            if let Ok(name) = std::str::from_utf8(key) {
                spaces.push(name.to_string());
            }
        }

        Ok(Output::SpaceNames(spaces))
    }

    fn add_space(&self, spacename: &str) -> Result<Output, Error> {
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
        Ok(Output::Success())
    }

    fn delete_space(&self, spacename: &str) -> Result<Output, Error> {
        let space_bytes = spacename.as_bytes();

        let mut txn = self.env.begin_rw_txn()?;
        txn.del(self.space, &space_bytes, None)
            .map_err(|e| match e {
                LmdbError::NotFound => Error::SpaceNotFound {
                    space_name: spacename.to_owned(),
                },
                _ => Error::from(e),
            })?;

        //Todo:keyからの削除

        //Todo:valueからの削除
        txn.commit()?;
        Ok(Output::Success())
    }

    fn show_keys(&self, spacename: &str) -> Result<Output, Error> {
        todo!()
    }

    fn add_key(&self, spacename: &str, keyname: &str, keytype: KeyType) -> Result<Output, Error> {
        let spacename_bytes = spacename.as_bytes();
        let mut txn = self.env.begin_rw_txn()?;

        //Spaceの存在の検証
        let space_id = txn.get(self.space, &spacename_bytes).map_err(|e| match e {
            LmdbError::NotFound => Error::SpaceNotFound {
                space_name: spacename.to_owned(),
            },
            _ => Error::from(e),
        })?;

        //Keyの作成
        let space_id_str = std::str::from_utf8(&space_id).map_err(|_| Error::NnKnown)?;

        let key_bytes = format!("{}:{}:{}", space_id_str, keyname, keytype_id(keytype));

        let key_id: [u8; 16] = *Uuid::new_v4().as_bytes();

        txn.put(self.key, &key_bytes, &key_id, lmdb::WriteFlags::empty())
            .map_err(|e| match e {
                LmdbError::KeyExist => Error::SpaceAlreadyExists {
                    space_name: spacename.to_owned(),
                },
                _ => Error::from(e),
            })?;

        txn.commit()?;
        Ok(Output::Success())
    }

    fn delete_key(&self, spacename: &str, keyname: &str) -> Result<Output, Error> {
        todo!()
    }

    fn transaction<F>(&self, cmds: Vec<F>) -> Result<Vec<Output>, Error>
    where
        F: Fn(&Self) -> Result<Output, Error>,
    {
        todo!()
    }

    fn info_space(&self, spacename: &str) -> Result<Output, Error> {
        todo!()
    }

    fn info_key(&self, spacename: &str, keyname: &str) -> Result<Output, Error> {
        todo!()
    }

    fn filter_value(
        &self,
        spacename: &str,
        keyname: &str,
        filter: FilterType,
    ) -> Result<Output, Error> {
        todo!()
    }

    fn get_value(
        &self,
        spacename: &str,
        keyname: &str,
        set: SpaceTimeIdSet,
    ) -> Result<Output, Error> {
        todo!()
    }

    fn set_value(
        &self,
        spacename: &str,
        keyname: &str,
        value: ValueEntry,
        set: SpaceTimeIdSet,
    ) -> Result<Output, Error> {
        todo!()
    }

    fn put_value(
        &self,
        spacename: &str,
        keyname: &str,
        value: ValueEntry,
        set: SpaceTimeIdSet,
    ) -> Result<Output, Error> {
        todo!()
    }

    fn delete_value(
        &self,
        spacename: &str,
        keyname: &str,
        set: SpaceTimeIdSet,
    ) -> Result<Output, Error> {
        todo!()
    }
}
