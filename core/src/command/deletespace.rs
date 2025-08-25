use crate::{
    error::Error,
    json::{input::DeleteSpace, output::Output},
};

#[cfg(feature = "default")]
use crate::io::kv::Storage;
#[cfg(feature = "wasm")]
use crate::io::memory::Storage;

pub fn deletespace(v: DeleteSpace, s: &mut Storage) -> Result<Output, Error> {
    s.delete_space(v)
}
