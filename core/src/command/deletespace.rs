use crate::{
    error::Error,
    json::{input::DeleteSpace, output::Output},
};

#[cfg(feature = "full")]
use crate::io::sled::Storage;
#[cfg(feature = "wasm")]
use crate::io::memory::Storage;

pub fn deletespace(v: DeleteSpace, s: &Storage) -> Result<Output, Error> {
    s.delete_space(v)
}
