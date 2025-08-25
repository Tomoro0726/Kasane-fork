use crate::{
    error::Error,
    io::kv::{self, Storage},
    json::{
        input::{AddSpace, DeleteSpace},
        output::Output,
    },
};

impl Storage {
    pub fn get_space(&mut self, name: &str) -> Result<u64, Error> {
        let manage = self.kv.open_tree("manage")?;
        let key = format!("s:{}", name);

        // Tree に存在するか確認
        let value = manage.get(&key)?.ok_or(Error::SpaceNotFound {
            space_name: name.to_string(),
            location: "Storage::get_space",
        })?;

        // 値は [u8; 8] に格納されていることを想定
        if value.len() != 8 {
            return Err(Error::ParseError {
                message: "Invalid space ID length".to_string(),
                location: "Storage::get_space",
            });
        }

        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&value);
        Ok(u64::from_be_bytes(bytes))
    }

    pub fn add_space(&mut self, v: AddSpace) -> Result<Output, Error> {
        let kv_key = format!("s:{}", v.spacename);
        let manage = self.kv.open_tree("manage")?;

        if manage.contains_key(&kv_key)? {
            return Err(Error::SpaceAlreadyExists {
                space_name: v.spacename,
                location: "Storage::add_space",
            });
        }

        // Db 本体から ID を生成
        let space_id = self.kv.generate_id()?.to_be_bytes();

        manage.insert(kv_key.as_bytes(), &space_id)?;
        Ok(Output::Success)
    }
    pub fn delete_space(&mut self, v: DeleteSpace) -> Result<Output, Error> {
        let manage = self.kv.open_tree("manage")?;
        let kv_key = format!("s:{}", v.spacename);

        if !manage.contains_key(&kv_key)? {
            return Err(Error::SpaceNotFound {
                space_name: v.spacename,
                location: "Storage::delete_space",
            });
        }

        manage.remove(&kv_key)?;

        Ok(Output::Success)
    }
    pub fn show_spaces(&self) -> Result<Output, Error> {
        // manage Tree を開く
        let manage = self.kv.open_tree("manage")?;

        // 全キーを走査してスペース名を抽出
        let mut spaces = Vec::new();
        for kv in manage.iter() {
            let (key, _value) = kv?;
            let key_str = String::from_utf8(key.to_vec()).map_err(|_| Error::ParseError {
                message: "Invalid UTF-8 key".to_string(),
                location: "Storage::show_spaces",
            })?;

            if let Some(name) = key_str.strip_prefix("s:") {
                spaces.push(name.to_string());
            }
        }

        Ok(Output::SpaceNames(spaces))
    }
}
