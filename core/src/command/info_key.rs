use std::sync::Arc;

use crate::{
    error::Error,
    io::{StorageTrait, full::Storage},
    json::{input::InfoKey, output::Output},
};

pub fn info_key(v: InfoKey, s: Arc<Storage>) -> Result<Output, Error> {
    s.info_key(&v.space_name, &v.key_name)
}
