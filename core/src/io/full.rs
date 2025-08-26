use sled::Db;
use sled::transaction::{
    ConflictableTransactionError, TransactionError, TransactionResult, Transactional,
};
use std::convert::TryInto;
use std::path::PathBuf;

use super::{Error, StorageTrait, ValueEntry};
use crate::io::tools::keytype_id::keytype_id;
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
        kv.open_tree("main").map_err(|e| e.to_string())?;
        Ok(Self { inner: kv })
    }
}

impl From<sled::Error> for Error {
    fn from(e: sled::Error) -> Self {
        Error::ParseError {
            message: format!("sled error: {}", e),
            location: "sled",
        }
    }
}
impl StorageTrait for Storage {
    fn show_spaces(&self) -> Result<Output, Error> {
        let tree = self.inner.open_tree("data").map_err(Error::from)?;
        let prefix = b"s:"; // space は "s:" + spaceid の形式
        let spaces: Vec<String> = tree
            .scan_prefix(prefix)
            .keys()
            .filter_map(|res| res.ok())
            .filter_map(|ivec| {
                let b = ivec.as_ref();
                Some(format!("{}", u64::from_le_bytes(b[2..10].try_into().ok()?)))
            })
            .collect();
        Ok(Output::SpaceNames(spaces))
    }

    fn add_space(&self, spacename: &str) -> Result<Output, Error> {
        let tree = self.inner.open_tree("data").map_err(Error::from)?;
        let spaceid = self.inner.generate_id().map_err(Error::from)?;
        let key = [&b"s:"[..], &spaceid.to_le_bytes()[..]].concat();
        if tree.get(&key)?.is_some() {
            return Err(Error::SpaceAlreadyExists {
                space_name: spacename.to_string(),
                location: "Storage::add_space",
            });
        }
        tree.insert(key, spacename.as_bytes())?;
        Ok(Output::Success())
    }

    fn delete_space(&self, spacename: &str) -> Result<Output, Error> {
        let tree = self.inner.open_tree("data").map_err(Error::from)?;
        let prefix = b"s:"; // space prefix

        // spaceid を検索
        let spaceid_opt = tree
            .scan_prefix(prefix)
            .filter_map(|res| res.ok())
            .find_map(|(k, v)| {
                if v.as_ref() == spacename.as_bytes() {
                    Some(k)
                } else {
                    None
                }
            });

        let spaceid_bytes = spaceid_opt.ok_or(Error::SpaceNotFound {
            space_name: spacename.to_string(),
            location: "Storage::delete_space",
        })?;

        let spaceid = u64::from_le_bytes(spaceid_bytes[2..10].try_into().unwrap());

        // トランザクションで space, key, value を削除
        let result: TransactionResult<(), Error> = tree.transaction(|t| {
            // key プレフィックス sID:keyID:keytypeID
            let key_prefix = [&b"k:"[..], &spaceid.to_le_bytes()[..]].concat();
            let keys_to_delete: Vec<Vec<u8>> = t
                .scan_prefix(&key_prefix)
                .keys()
                .filter_map(|res| res.ok())
                .map(|ivec| ivec.to_vec())
                .collect();

            for k in &keys_to_delete {
                // value prefix: sID:keyID:keytypeID:
                let value_prefix = [&b"v:"[..], k.as_slice(), &b":"[..]].concat();
                let vals_to_delete: Vec<Vec<u8>> = t
                    .scan_prefix(&value_prefix)
                    .keys()
                    .filter_map(|res| res.ok())
                    .map(|ivec| ivec.to_vec())
                    .collect();
                for v in vals_to_delete {
                    t.remove(v)?;
                }
                t.remove(k)?;
            }

            // space 削除
            t.remove(spaceid_bytes)?;

            Ok(())
        });

        match result {
            Ok(_) => Ok(Output::Success()),
            Err(TransactionError::Abort(e)) => Err(e),
            Err(TransactionError::Storage(e)) => Err(Error::from(e)),
        }
    }

    fn show_keys(&self, spacename: &str) -> Result<Output, Error> {
        let tree = self.inner.open_tree("data").map_err(Error::from)?;
        // spaceid を取得
        let prefix = b"s:";
        let spaceid_bytes = tree
            .scan_prefix(prefix)
            .filter_map(|res| res.ok())
            .find_map(|(k, v)| {
                if v.as_ref() == spacename.as_bytes() {
                    Some(k)
                } else {
                    None
                }
            })
            .ok_or(Error::SpaceNotFound {
                space_name: spacename.to_string(),
                location: "Storage::show_keys",
            })?;
        let spaceid = u64::from_le_bytes(spaceid_bytes[2..10].try_into().unwrap());

        // key を取得
        let key_prefix = [&b"k:"[..], &spaceid.to_le_bytes()[..]].concat();
        let keys: Vec<String> = tree
            .scan_prefix(&key_prefix)
            .keys()
            .filter_map(|res| res.ok())
            .filter_map(|k| {
                let k_bytes = k.as_ref();
                // k: "k:" + spaceid(8) + keyID(8) + keytype(1)
                if k_bytes.len() >= 17 {
                    Some(format!(
                        "{}",
                        u64::from_le_bytes(k_bytes[10..18].try_into().ok()?)
                    ))
                } else {
                    None
                }
            })
            .collect();

        Ok(Output::KeyNames(keys))
    }

    fn add_key(&self, spacename: &str, keyname: &str, keytype: KeyType) -> Result<Output, Error> {
        let tree = self.inner.open_tree("data").map_err(Error::from)?;
        let keytype_id = keytype_id(keytype);

        // spaceid を取得
        let prefix = b"s:";
        let spaceid_bytes = tree
            .scan_prefix(prefix)
            .filter_map(|res| res.ok())
            .find_map(|(k, v)| {
                if v.as_ref() == spacename.as_bytes() {
                    Some(k)
                } else {
                    None
                }
            })
            .ok_or(Error::SpaceNotFound {
                space_name: spacename.to_string(),
                location: "Storage::add_key",
            })?;
        let spaceid = u64::from_le_bytes(spaceid_bytes[2..10].try_into().unwrap());

        // keyID を生成
        let keyid = self.inner.generate_id().map_err(Error::from)?;
        let key_bytes = [
            &b"k:"[..],
            &spaceid.to_le_bytes()[..],
            &keyid.to_le_bytes()[..],
            &[keytype_id],
        ]
        .concat();
        if tree.get(&key_bytes)?.is_some() {
            return Err(Error::KeyAlreadyExists {
                key_name: keyname.to_string(),
                space_name: spacename.to_string(),
                location: "Storage::add_key",
            });
        }

        tree.insert(key_bytes, keyname.as_bytes())?;
        Ok(Output::Success())
    }

    fn delete_key(&self, spacename: &str, keyname: &str) -> Result<Output, Error> {
        let tree = self.inner.open_tree("data").map_err(Error::from)?;
        let prefix = b"s:";
        // spaceid を取得
        let spaceid_bytes = tree
            .scan_prefix(prefix)
            .filter_map(|res| res.ok())
            .find_map(|(k, v)| {
                if v.as_ref() == spacename.as_bytes() {
                    Some(k)
                } else {
                    None
                }
            })
            .ok_or(Error::SpaceNotFound {
                space_name: spacename.to_string(),
                location: "Storage::delete_key",
            })?;
        let spaceid = u64::from_le_bytes(spaceid_bytes[2..10].try_into().unwrap());

        // key を検索して削除
        let key_prefix = [&b"k:"[..], &spaceid.to_le_bytes()[..]].concat();
        let keys_to_delete: Vec<Vec<u8>> = tree
            .scan_prefix(&key_prefix)
            .filter_map(|res| res.ok())
            .filter_map(|(k, v)| {
                if v.as_ref() == keyname.as_bytes() {
                    Some(k.to_vec())
                } else {
                    None
                }
            })
            .collect();

        if keys_to_delete.is_empty() {
            return Err(Error::KeyNotFound {
                key_name: keyname.to_string(),
                space_name: spacename.to_string(),
                location: "Storage::delete_key",
            });
        }

        for k in keys_to_delete {
            tree.remove(k)?;
        }

        Ok(Output::Success())
    }

    fn info_space(&self, _: &str) -> Result<Output, Error> {
        todo!()
    }
    fn info_key(&self, _: &str, _: &str) -> Result<Output, Error> {
        todo!()
    }
    fn filter_value(&self, _: &str, _: &str, _: FilterType) -> Result<Output, Error> {
        todo!()
    }
    fn get_value(&self, _: &str, _: &str, _: SpaceTimeIdSet) -> Result<Output, Error> {
        todo!()
    }
    fn set_value(
        &self,
        _: &str,
        _: &str,
        _: ValueEntry,
        _: SpaceTimeIdSet,
    ) -> Result<Output, Error> {
        todo!()
    }
    fn put_value(
        &self,
        _: &str,
        _: &str,
        _: ValueEntry,
        _: SpaceTimeIdSet,
    ) -> Result<Output, Error> {
        todo!()
    }
    fn delete_value(&self, _: &str, _: &str, _: SpaceTimeIdSet) -> Result<Output, Error> {
        todo!()
    }

    fn transaction<F>(&self, cmds: Vec<F>) -> Result<Vec<Output>, Error>
    where
        F: Fn(&Self) -> Result<Output, Error>,
    {
        todo!()
    }
}
