use std::sync::Arc;

use crate::{
    error::Error,
    io::{Storage, StorageTrait},
    json::{input::DeleteKey, output::Output},
};

pub fn deletekey(v: DeleteKey, s: Arc<Storage>) -> Result<Output, Error> {
    s.delete_key(&v.spacename, &v.keyname)
}
