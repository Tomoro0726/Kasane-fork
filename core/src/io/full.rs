use sled::transaction::TransactionResult;
use sled::transaction::{ConflictableTransactionError, TransactionError, Transactional};
use std::path::PathBuf;
// 外部クレート
use sled::Db;

// 内部モジュール
use super::{Error, StorageTrait, ValueEntry};
use crate::json::input::{FilterType, KeyType};
use crate::json::output::Output;
use kasane_logic::set::SpaceTimeIdSet;

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

impl From<sled::Error> for Error {
    fn from(e: sled::Error) -> Self {
        match e {
            sled::Error::Io(err) => Error::ParseError {
                message: format!("IO error: {}", err),
                location: "sled IO",
            },
            sled::Error::Unsupported(_) => Error::ParseError {
                message: "Unsupported sled operation".to_string(),
                location: "sled",
            },
            sled::Error::ReportableBug(msg) => Error::ParseError {
                message: format!("Sled internal bug: {}", msg),
                location: "sled",
            },
            _ => Error::ParseError {
                message: format!("Unknown sled error: {}", e),
                location: "sled",
            },
        }
    }
}

impl StorageTrait for Storage {
    fn transaction<F>(&self, cmds: Vec<F>) -> Result<Vec<Output>, Error>
    where
        F: Fn(&Self) -> Result<Output, Error>,
    {
        todo!()
    }

    fn show_spaces(&self) -> Result<Output, Error> {
        // space用のTreeを開く
        let tree = self.inner.open_tree("space").map_err(Error::from)?; // sled::Error -> Error に変換

        // すべてのキーを収集
        let spaces: Vec<String> = tree
            .iter()
            .keys()
            .filter_map(|res| res.ok()) // sled::Result<IVec> -> IVec
            .filter_map(|key| String::from_utf8(key.to_vec()).ok()) // &[u8] -> String
            .collect();

        Ok(Output::SpaceNames(spaces))
    }

    fn add_space(&self, spacename: &str) -> Result<Output, Error> {
        let space_tree = self.inner.open_tree("space").map_err(Error::from)?;

        // すでに存在するか確認
        if space_tree.get(spacename.as_bytes())?.is_some() {
            return Err(Error::SpaceAlreadyExists {
                space_name: spacename.to_string(),
                location: "Storage::add_space",
            });
        }

        // ユニークIDを生成
        let spaceid = self.inner.generate_id().map_err(Error::from)?;

        // spacename → spaceid を登録
        space_tree.insert(spacename.as_bytes(), &spaceid.to_le_bytes())?;

        Ok(Output::Success())
    }

    fn delete_space(&self, spacename: &str) -> Result<Output, Error> {
        todo!()
    }

    fn show_keys(&self, spacename: &str) -> Result<Output, Error> {
        todo!()
    }

    fn info_space(&self, spacename: &str) -> Result<Output, Error> {
        todo!()
    }

    fn add_key(&self, spacename: &str, keyname: &str, keytype: KeyType) -> Result<Output, Error> {
        let key_tree = self.inner.open_tree("key").map_err(Error::from)?;
        let space_tree = self.inner.open_tree("space").map_err(Error::from)?;

        let keytype_id: u8 = match keytype {
            KeyType::INT => 1,
            KeyType::BOOLEAN => 2,
            KeyType::TEXT => 3,
            KeyType::FLOAT => 4,
        };

        let keyname = keyname.to_string();
        let spacename = spacename.to_string();

        let result: TransactionResult<(), Error> =
            (&key_tree, &space_tree).transaction(|(ktree, stree)| {
                // spacename から spaceid を取得
                let spaceid_bytes =
                    stree
                        .get(spacename.as_bytes())?
                        .ok_or(ConflictableTransactionError::Abort(Error::SpaceNotFound {
                            space_name: spacename.clone(),
                            location: "Storage::add_key",
                        }))?;

                let spaceid = u64::from_le_bytes(spaceid_bytes.as_ref().try_into().unwrap());

                // キー組み立て: spaceid:keyname
                let key_str = format!("{}:{}", spaceid, keyname);

                if ktree.get(key_str.as_bytes())?.is_some() {
                    return Err(ConflictableTransactionError::Abort(
                        Error::KeyAlreadyExists {
                            key_name: keyname.clone(),
                            space_name: spacename.clone(),
                            location: "Storage::add_key",
                        },
                    ));
                }

                // key作成
                ktree.insert(key_str.as_bytes(), &[keytype_id])?;

                Ok(())
            });

        match result {
            Ok(_) => Ok(Output::Success()),
            Err(TransactionError::Abort(e)) => Err(e),
            Err(TransactionError::Storage(e)) => Err(Error::from(e)),
        }
    }

    fn delete_key(&self, spacename: &str, keyname: &str) -> Result<Output, Error> {
        let key_tree = self.inner.open_tree("key").map_err(Error::from)?;
        let space_tree = self.inner.open_tree("space").map_err(Error::from)?;

        let keyname = keyname.to_string();
        let spacename = spacename.to_string();

        let result = (&key_tree, &space_tree).transaction(|(ktree, stree)| {
            // spaceid を取得
            let spaceid_bytes =
                stree
                    .get(spacename.as_bytes())?
                    .ok_or(ConflictableTransactionError::Abort(Error::SpaceNotFound {
                        space_name: spacename.clone(),
                        location: "Storage::delete_key",
                    }))?;

            let spaceid = u64::from_le_bytes(spaceid_bytes.as_ref().try_into().unwrap());

            let key_str = format!("{}:{}", spaceid, keyname);

            if ktree.get(key_str.as_bytes())?.is_none() {
                return Err(ConflictableTransactionError::Abort(Error::KeyNotFound {
                    key_name: keyname.clone(),
                    space_name: spacename.clone(),
                    location: "Storage::delete_key",
                }));
            }

            ktree.remove(key_str.as_bytes())?;
            Ok(())
        });

        match result {
            Ok(_) => Ok(Output::Success()),
            Err(TransactionError::Abort(e)) => Err(e),
            Err(TransactionError::Storage(e)) => Err(Error::from(e)),
        }
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
