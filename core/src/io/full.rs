use std::{env, path::PathBuf};

use crate::{
    io::StorageTrait,
    json::output::{InfoKey, InfoSpace, InfoUser, Output, ShowUsers, Showkeys},
};
use argon2::password_hash::PasswordHasher;
use argon2::{Argon2, PasswordHash, PasswordVerifier, password_hash::SaltString};
use lmdb::{Cursor, DatabaseFlags, Error as LmdbError};
use rand::rngs::OsRng;

use super::Error;
use lmdb::{Database, Environment, Transaction};
use uuid::Uuid;

pub struct Storage {
    pub space: Database,
    pub key: Database,
    pub value: Database,
    pub user: Database,
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
                db_name: "unknown",
                location: "unknown",
            },
            _ => Error::LmdbError {
                message: e.to_string(),
                location: "unknown",
            },
        }
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(e: std::str::Utf8Error) -> Self {
        Error::ParseError {
            message: format!("Invalid UTF-8: {}", e),
            location: "unknown",
        }
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Error::ParseError {
            message: format!("Invalid UTF-8 (from Vec<u8>): {}", e),
            location: "unknown",
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
        let user = env.create_db(Some("user"), DatabaseFlags::empty())?;

        let storage = Self {
            space,
            key,
            value,
            user,
            env,
        };

        // === 初回起動時の admin ユーザー作成 ===
        {
            let txn = storage.env.begin_ro_txn()?;
            let admin_exists = txn.get(storage.user, b"admin").is_ok();
            drop(txn);

            if !admin_exists {
                // デフォルトパスワードは "admin" にしておく
                // 必要なら env から読み込むことも可能
                storage.create_user("admin", "admin")?;
                println!(
                    "✔ 初回起動: admin ユーザーを作成しました (username=admin, password=admin)"
                );
            }
        }

        Ok(storage)
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
        let space_bytes = spacename.as_bytes();
        let mut txn = self.env.begin_rw_txn()?;

        // 1. Space の削除
        txn.del(self.space, &space_bytes, None)
            .map_err(|e| match e {
                LmdbError::NotFound => Error::SpaceNotFound {
                    space_name: spacename.to_string(),
                },
                _ => Error::from(e),
            })?;

        // 2. Space に属するキーを先にコピー
        let mut keys_to_delete = Vec::new();
        {
            let mut cursor = txn.open_ro_cursor(self.key)?; // 読み取り専用カーソルで走査
            let prefix = space_bytes; // space UUID が key のプレフィックス
            for result in cursor.iter_start() {
                let (k, _) = result;
                if k.starts_with(prefix) {
                    keys_to_delete.push(k.to_vec()); // Vec にコピー
                }
            }
        }

        // 3. コピーしたキーを削除
        for k in keys_to_delete {
            txn.del(self.key, &k, None)?;
        }

        txn.commit()?;
        Ok(Output::Success)
    }

    fn info_space(&self, spacename: &str) -> Result<crate::json::output::Output, Error> {
        // 1. Space の存在確認
        let space_bytes = spacename.as_bytes();
        let txn = self.env.begin_ro_txn()?;
        let space_uuid = match txn.get(self.space, &space_bytes) {
            Ok(v) => std::str::from_utf8(v)?,
            Err(LmdbError::NotFound) => {
                return Err(Error::SpaceNotFound {
                    space_name: spacename.to_string(),
                });
            }
            Err(e) => return Err(Error::from(e)),
        };

        // 2. キーを走査してこのスペースに属するものだけ抽出
        let mut cursor = txn.open_ro_cursor(self.key)?;
        let mut keys_info: Vec<InfoKey> = Vec::new();
        let prefix = format!("{}:", space_uuid);

        for result in cursor.iter_start() {
            let (k, _v) = result;
            if k.starts_with(prefix.as_bytes()) {
                let key_str = std::str::from_utf8(k)?;
                let parts: Vec<&str> = key_str.split(':').collect();
                if parts.len() != 4 {
                    return Err(Error::ParseError {
                        message: format!("Invalid key format: {}", key_str),
                        location: "io::info_space",
                    });
                }

                keys_info.push(InfoKey {
                    keyname: parts[1].to_string(),
                    keytype: parts[2].to_string(),
                    keymode: parts[3].to_string(),
                });
            }
        }

        let info = InfoSpace {
            spacename: spacename.to_string(),
            keynames: keys_info,
        };

        Ok(crate::json::output::Output::InfoSpace(info))
    }

    fn show_spaces(&self) -> Result<crate::json::output::Output, Error> {
        let txn = self.env.begin_ro_txn()?; // 読み取り専用トランザクション
        let mut cursor = txn.open_ro_cursor(self.space)?; // space DB のカーソルを開く

        let mut spaces = Vec::new();

        for result in cursor.iter_start() {
            let (key_bytes, _val_bytes) = result;

            // &[u8] -> &str への変換。? で Error に変換可能
            let s: &str = std::str::from_utf8(key_bytes)?;

            // &str -> String
            let string: String = s.to_string();

            spaces.push(string);
        }

        Ok(Output::ShowSpaces(crate::json::output::ShowSpaces {
            spacenames: spaces,
        }))
    }

    fn create_key(
        &self,
        spacename: &str,
        keyname: &str,
        keytype: crate::json::input::KeyType,
        keymode: crate::json::input::KeyMode,
    ) -> Result<crate::json::output::Output, Error> {
        // Spaceの存在を確認
        let space_bytes = spacename.as_bytes();
        let mut txn = self.env.begin_rw_txn()?;
        let space_uuid = match txn.get(self.space, &space_bytes) {
            Ok(v) => std::str::from_utf8(v)?,
            Err(LmdbError::NotFound) => {
                return Err(Error::SpaceNotFound {
                    space_name: spacename.to_string(),
                });
            }
            Err(e) => return Err(Error::from(e)),
        };

        // Key用バイト列
        let key_bytes = format!("{}:{}:{:?}:{:?}", space_uuid, keyname, keytype, keymode);

        let key_id: [u8; 16] = *Uuid::new_v4().as_bytes();

        txn.put(
            self.key,
            &key_bytes.as_bytes(),
            &key_id,
            lmdb::WriteFlags::empty(),
        )
        .map_err(|e| match e {
            LmdbError::KeyExist => Error::KeyAlreadyExists {
                space_name: spacename.to_owned(),
                key_name: keyname.to_owned(),
                location: "io::create_key",
            },
            e => Error::from(e),
        })?;

        txn.commit()?;
        Ok(Output::Success)
    }

    fn drop_key(
        &self,
        spacename: &str,
        keyname: &str,
    ) -> Result<crate::json::output::Output, Error> {
        let space_bytes = spacename.as_bytes();
        let mut txn = self.env.begin_rw_txn()?;
        let space_uuid = match txn.get(self.space, &space_bytes) {
            Ok(v) => std::str::from_utf8(v)?,
            Err(LmdbError::NotFound) => {
                return Err(Error::SpaceNotFound {
                    space_name: spacename.to_string(),
                });
            }
            Err(e) => return Err(Error::from(e)),
        };

        // Keyのprefix
        let key_prefix = format!("{}:{}", space_uuid, keyname);

        match txn.del(self.key, &key_prefix.as_bytes(), None) {
            Ok(_) => {
                txn.commit()?;
                Ok(Output::Success)
            }
            Err(LmdbError::NotFound) => Err(Error::KeyNotFound {
                space_name: spacename.to_owned(),
                key_name: keyname.to_owned(),
                location: "io::drop_key",
            }),
            Err(e) => Err(Error::from(e)),
        }
    }

    fn show_keys(&self, spacename: &str) -> Result<crate::json::output::Output, Error> {
        let space_bytes = spacename.as_bytes();
        let txn = self.env.begin_ro_txn()?;
        let space_uuid = match txn.get(self.space, &space_bytes) {
            Ok(v) => std::str::from_utf8(v)?,
            Err(LmdbError::NotFound) => {
                return Err(Error::SpaceNotFound {
                    space_name: spacename.to_string(),
                });
            }
            Err(e) => return Err(Error::from(e)),
        };

        let mut cursor = txn.open_ro_cursor(self.key)?;
        let prefix = format!("{}:", space_uuid);
        let mut keys = Vec::new();

        for result in cursor.iter_start() {
            let (k, _v) = result;
            if k.starts_with(prefix.as_bytes()) {
                // prefixを除いたkey名を抽出
                let key_str = std::str::from_utf8(&k[prefix.len()..])?.to_string();
                keys.push(key_str);
            }
        }

        Ok(Output::Showkeys(Showkeys { keynames: keys }))
    }

    fn info_key(
        &self,
        spacename: &str,
        keyname: &str,
    ) -> Result<crate::json::output::Output, Error> {
        // 1. Space の存在確認
        let space_bytes = spacename.as_bytes();
        let txn = self.env.begin_ro_txn()?;
        let space_uuid = match txn.get(self.space, &space_bytes) {
            Ok(v) => std::str::from_utf8(v)?,
            Err(LmdbError::NotFound) => {
                return Err(Error::SpaceNotFound {
                    space_name: spacename.to_string(),
                });
            }
            Err(e) => return Err(Error::from(e)),
        };

        // 2. Key の prefix を作成
        let key_prefix = format!("{}:{}", space_uuid, keyname);
        let mut cursor = txn.open_ro_cursor(self.key)?;

        for result in cursor.iter_start() {
            let (k, _v) = result;
            if k.starts_with(key_prefix.as_bytes()) {
                // k は "space_uuid:keyname:keytype:keymode" 形式
                let key_str = std::str::from_utf8(k)?;
                let parts: Vec<&str> = key_str.split(':').collect();
                if parts.len() != 4 {
                    return Err(Error::ParseError {
                        message: format!("Invalid key format: {}", key_str),
                        location: "io::info_key",
                    });
                }

                let info = InfoKey {
                    keyname: parts[1].to_string(),
                    keytype: parts[2].to_string(),
                    keymode: parts[3].to_string(),
                };

                return Ok(crate::json::output::Output::InfoKey(info));
            }
        }

        Err(Error::KeyNotFound {
            space_name: spacename.to_string(),
            key_name: keyname.to_string(),
            location: "io::info_key",
        })
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

    fn create_user(&self, username: &str, password: &str) -> Result<Output, Error> {
        let mut txn = self.env.begin_rw_txn()?;

        // ユーザー名が既に存在するか確認
        if txn.get(self.user, &username.as_bytes()).is_ok() {
            return Err(Error::UserAlreadyExists {
                user_name: username.to_string(),
            });
        }

        // ソルトを生成
        let salt = SaltString::generate(&mut OsRng);

        // Argon2 インスタンスを作成
        let argon2 = Argon2::default();

        // パスワードをハッシュ化
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| Error::LmdbError {
                message: "Password hash failed".to_string(),
                location: "create_user",
            })?
            .to_string();

        // ユーザー名とハッシュを保存
        txn.put(
            self.user,
            &username.as_bytes(),
            &password_hash.as_bytes(),
            lmdb::WriteFlags::empty(),
        )?;
        txn.commit()?;

        Ok(Output::Success)
    }

    fn drop_user(&self, username: &str) -> Result<Output, Error> {
        if username == "admin" {
            return Err(Error::UserNotFound {
                user_name: "admin".to_string(),
            }); // もしくは専用のエラーを作っても良い
        }

        let mut txn = self.env.begin_rw_txn()?;
        match txn.del(self.user, &username.as_bytes(), None) {
            Ok(_) => {
                txn.commit()?;
                Ok(Output::Success)
            }
            Err(LmdbError::NotFound) => Err(Error::UserNotFound {
                user_name: username.to_string(),
            }),
            Err(e) => Err(Error::from(e)),
        }
    }

    fn info_user(&self, username: &str) -> Result<Output, Error> {
        let txn = self.env.begin_ro_txn()?;
        let hash_bytes =
            txn.get(self.user, &username.as_bytes())
                .map_err(|_| Error::UserNotFound {
                    user_name: username.to_string(),
                })?;
        Ok(Output::InfoUser(InfoUser {
            user_name: username.to_string(),
        }))
    }
    fn show_users(&self) -> Result<Output, Error> {
        let txn = self.env.begin_ro_txn()?;
        let mut cursor = txn.open_ro_cursor(self.user)?;

        let mut users = Vec::new();

        for result in cursor.iter_start() {
            let (key_bytes, _value_bytes) = result;
            let username = std::str::from_utf8(key_bytes)?.to_string();
            users.push(username);
        }

        Ok(Output::ShowUsers(ShowUsers { users }))
    }

    fn verify_user(&self, username: &str, password: &str) -> Result<bool, Error> {
        let txn = self.env.begin_ro_txn()?;

        // ユーザー名に対応するハッシュを取得
        let hash_bytes = match txn.get(self.user, &username.as_bytes()) {
            Ok(v) => v,
            Err(_) => return Ok(false), // ユーザーが存在しない場合は false
        };

        // ハッシュ文字列に変換
        let hash_str = std::str::from_utf8(hash_bytes)?;

        // PasswordHash に変換
        let parsed_hash = PasswordHash::new(hash_str).map_err(|_| Error::LmdbError {
            message: "Invalid stored password hash".to_string(),
            location: "verify_user",
        })?;

        // Argon2 インスタンス
        let argon2 = Argon2::default();

        // パスワードを検証
        let valid = argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok();

        Ok(valid)
    }
}
