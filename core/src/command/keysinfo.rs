use std::sync::Arc;

use crate::{
    error::Error,
    io::Storage,
    json::{input::KeysInfo, output::Output},
};

pub fn keysinfo(v: KeysInfo, s: Arc<Storage>) -> Result<Output, Error> {
    Ok(s.keys_info(v))
}
