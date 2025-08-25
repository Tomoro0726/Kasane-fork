use crate::{
    error::Error,
    json::{input::Keys, output::Output},
};

#[cfg(feature = "full")]
use crate::io::kv::Storage;
#[cfg(feature = "wasm")]
use crate::io::memory::Storage;

pub fn showkeys(v: Keys, s: &mut Storage) -> Result<Output, Error> {
    Ok(s.show_keys(v))
}
