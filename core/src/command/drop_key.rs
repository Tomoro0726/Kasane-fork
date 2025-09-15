use std::sync::Arc;

use crate::{
    error::Error,
    io::{Storage, StorageTrait},
    json::{input::DropKey, output::Output},
};

pub fn drop_key(v: DropKey, s: Arc<Storage>) -> Result<Output, Error> {
    s.delete_key(&v.space_name, &v.key_name)
}
