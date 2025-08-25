use crate::{
    error::Error,
    json::{input::KeysInfo, output::Output},
};

#[cfg(feature = "default")]
use crate::io::kv::Storage;
#[cfg(feature = "wasm")]
use crate::io::memory::Storage;

pub fn keysinfo(v: KeysInfo, s: &mut Storage) -> Result<Output, Error> {
    Ok(s.keys_info(v))
}
