use lmdb::Error as LmdbError;
use lmdb::{Database, DatabaseFlags, Environment, Transaction, WriteFlags};
use uuid::Uuid;

use crate::{
    error::Error,
    io::{Storage, StorageTrait},
    json::output::Output,
};

impl StorageTrait for Storage {
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
}
