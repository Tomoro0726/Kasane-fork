use std::sync::Arc;

use crate::{
    error::Error,
    io::{Storage, StorageTrait},
    json::{input::Keys, output::Output},
};

pub fn showkeys(v: Keys, s: Arc<Storage>) -> Result<Output, Error> {
    s.show_keys(&v.spacename)
}
