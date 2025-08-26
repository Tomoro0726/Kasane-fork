use crate::{error::Error, json::output::Output};

#[cfg(feature = "full")]
use crate::io::sled::Storage;
#[cfg(feature = "wasm")]
use crate::io::memory::Storage;

pub fn showspaces(s: & Storage) -> Result<Output, Error> {
    s.show_spaces()
}
