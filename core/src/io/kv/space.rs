use crate::{
    error::Error,
    io::kv::{Storage, space},
    json::{
        input::{AddKey, DeleteKey},
        output::Output,
    },
};

impl Storage {
    pub fn add_key(&mut self, v: AddKey) -> Result<Output, Error> {
        let space_id = self.get_space(&v.spacename)?;
        let kv_key = format!("k:{}:{}:{:?}", space_id, v.keyname, v.r#type);
        let manage = self.kv.open_tree("manage")?;

        if manage.contains_key(&kv_key)? {
            return Err(Error::KeyAlreadyExists {
                key_name: v.keyname,
                space_name: v.spacename,
                location: "Storage::add_key",
            });
        }
        let key_id = self.kv.generate_id()?.to_be_bytes();
        manage.insert(kv_key.as_bytes(), &key_id)?;
        Ok(Output::Success)
    }

    pub fn delete_key(&mut self, v: DeleteKey) -> Result<Output, Error> {
        // まずスペースIDを取得
        let space_id = self.get_space(&v.spacename)?;

        // キー名に対応する sled のキーを作成
        let kv_key = format!("k:{}:{}", space_id, v.keyname);

        // manage Tree を開く
        let manage = self.kv.open_tree("manage")?;

        // キーが存在するか確認
        if !manage.contains_key(&kv_key)? {
            return Err(Error::KeyNotFound {
                key_name: v.keyname,
                space_name: v.spacename,
                location: "Storage::delete_key",
            });
        }
        manage.remove(kv_key.as_bytes())?;
        Ok(Output::Success)
    }
}
