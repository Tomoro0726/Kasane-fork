use crate::{error::Error, json::output::Output};

#[cfg(feature = "default")]
use crate::io::kv::Storage;
#[cfg(feature = "wasm")]
use crate::io::memory::Storage;

pub fn showspaces(s: &mut Storage) -> Result<Output, Error> {
    Ok(s.show_spaces())
}
