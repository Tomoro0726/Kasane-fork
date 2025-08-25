use crate::error::Error;
use crate::io::memory::{Key, Space, Storage};
use crate::json::input::{AddKey, DeleteKey, Keys, KeysInfo};
use crate::json::output::{KeyInfoOutput, Output};

impl Storage {
    pub fn get_key(&mut self, spacename: &str, keyname: &str) -> Result<&mut Key, Error> {
        let space =
            self.space
                .iter_mut()
                .find(|s| s.name == spacename)
                .ok_or(Error::SpaceNotFound {
                    space_name: spacename.to_string(),
                    location: "io::storage::get_key",
                })?;

        space
            .key
            .iter_mut()
            .find(|k| k.name == keyname)
            .ok_or(Error::KeyNotFound {
                key_name: keyname.to_string(),
                space_name: spacename.to_string(),
                location: "io::storage::get_key",
            })
    }

    /// 指定スペース内のキー一覧を表示
    pub fn show_keys(&self, v: Keys) -> Output {
        match self.space.iter().find(|s| s.name == v.spacename) {
            Some(space) => {
                let names = space.key.iter().map(|k| k.name.clone()).collect();
                Output::KeyNames(names)
            }
            None => Output::KeyNames(Vec::new()),
        }
    }

    /// キーを追加
    pub fn add_key(&mut self, v: AddKey) -> Result<Output, Error> {
        let space = self
            .space
            .iter_mut()
            .find(|s| s.name == v.spacename)
            .ok_or(Error::SpaceNotFound {
                space_name: v.spacename.clone(),
                location: "io::space::add_key",
            })?;

        if space.key.iter().any(|k| k.name == v.keyname) {
            return Err(Error::KeyAlreadyExists {
                key_name: v.keyname,
                space_name: v.spacename,
                location: "io::space::add_key",
            });
        }

        space.key.push(Key {
            name: v.keyname,
            r#type: v.r#type,
            value: Vec::new(),
        });

        Ok(Output::Success)
    }

    /// キーを削除
    pub fn delete_key(&mut self, v: DeleteKey) -> Result<Output, Error> {
        let space = self
            .space
            .iter_mut()
            .find(|s| s.name == v.spacename)
            .ok_or(Error::SpaceNotFound {
                space_name: v.spacename.clone(),
                location: "io::space::delete_key",
            })?;

        if !space.key.iter().any(|k| k.name == v.name) {
            return Err(Error::KeyNotFound {
                key_name: v.name,
                space_name: v.spacename,
                location: "io::space::delete_key",
            });
        }

        space.key.retain(|k| k.name != v.name);
        Ok(Output::Success)
    }

    /// キー情報を取得
    pub fn keys_info(&self, v: KeysInfo) -> Output {
        match self.space.iter().find(|s| s.name == v.spacename) {
            Some(space) => {
                let info = space
                    .key
                    .iter()
                    .map(|k| KeyInfoOutput {
                        keyname: k.name.clone(),
                        keytype: k.r#type,
                    })
                    .collect();
                Output::KeysInfo(info)
            }
            None => Output::KeysInfo(Vec::new()),
        }
    }
}
