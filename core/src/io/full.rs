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
        let spacename_bytes = spacename.as_bytes();
        let txn = self.env.begin_ro_txn()?; // 読み取り専用トランザクション

        // 1. Space の存在確認
        let space_id = txn.get(self.space, &spacename_bytes).map_err(|e| match e {
            lmdb::Error::NotFound => Error::SpaceNotFound {
                space_name: spacename.to_owned(),
            },
            _ => Error::from(e),
        })?;

        // 2. カーソルでキーを走査
        let mut keys = Vec::new();
        {
            let mut cursor = txn.open_ro_cursor(self.key)?;
            let prefix = &space_id[..]; // space_id で始まるキーのみ取得

            for result in cursor.iter_start() {
                let (key_bytes, _) = result;
                if key_bytes.starts_with(prefix) {
                    // space_id の後に続く部分を keyname として取得
                    let keyname_bytes = &key_bytes[prefix.len()..];
                    if let Ok(keyname_str) = std::str::from_utf8(keyname_bytes) {
                        keys.push(keyname_str.to_string());
                    }
                }
            }
        } // cursor がここでドロップされる

        Ok(Output::KeyNames(keys))
    }

    fn add_key(&self, spacename: &str, keyname: &str, keytype: KeyType) -> Result<Output, Error> {
        let spacename_bytes = spacename.as_bytes();
        let mut txn = self.env.begin_rw_txn()?;

        // 1. Space の存在確認
        let space_id = txn.get(self.space, &spacename_bytes).map_err(|e| match e {
            LmdbError::NotFound => Error::SpaceNotFound {
                space_name: spacename.to_owned(),
            },
            _ => Error::from(e),
        })?;

        // 2. Key の構成
        let key_bytes = key_bytes(&space_id, keyname, keytype)?;

        // 3. 既存チェック
        if txn.get(self.key, &key_bytes.as_bytes()).is_ok() {
            return Err(Error::KeyAlreadyExists {
                key_name: keyname.to_owned(),
                space_name: spacename.to_owned(),
                location: "a",
            });
        }

        // 4. Key を追加
        let key_id: [u8; 16] = *Uuid::new_v4().as_bytes();
        txn.put(
            self.key,
            &key_bytes.as_bytes(),
            &key_id,
            lmdb::WriteFlags::empty(),
        )?;

        txn.commit()?;
        Ok(Output::Success())
    }

    fn delete_key(&self, spacename: &str, keyname: &str) -> Result<Output, Error> {
        let spacename_bytes = spacename.as_bytes();
        let mut txn = self.env.begin_rw_txn()?;

        // 1. Space の存在確認
        let space_id = txn.get(self.space, &spacename_bytes).map_err(|e| match e {
            lmdb::Error::NotFound => Error::SpaceNotFound {
                space_name: spacename.to_owned(),
            },
            _ => Error::from(e),
        })?;

        // 2. 削除対象 key の prefix を作成
        let prefix = [&space_id[..], keyname.as_bytes()].concat();

        // 3. カーソルスコープを限定
        let found = {
            let mut cursor = txn.open_rw_cursor(self.key)?;
            let mut found_flag = false;

            for result in cursor.iter_start() {
                let (key_bytes, _) = result;
                if key_bytes.starts_with(&prefix) {
                    cursor.del(lmdb::WriteFlags::empty())?;
                    found_flag = true;
                }
            }

            found_flag
        };

        if !found {
            return Err(Error::KeyNotFound {
                space_name: spacename.to_owned(),
                key_name: keyname.to_owned(),
                location: "delete_key",
            });
        }

        txn.commit()?;
        Ok(Output::Success())
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
