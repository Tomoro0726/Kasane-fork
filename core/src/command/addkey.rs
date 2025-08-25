use crate::json::input::AddKey;
use crate::json::output::Output;
use crate::{command::tools::valid_name::valid_name, error::Error};

#[cfg(feature = "full")]
use crate::io::kv::Storage;
#[cfg(feature = "wasm")]
use crate::io::memory::Storage;

pub fn addkey(v: AddKey, s: &mut Storage) -> Result<Output, Error> {
    if !valid_name(&v.keyname) {
        Err(Error::KeyNameValidationError {
            name: v.keyname,
            reason: "only a-z, A-Z, 0-9, - _ . @ + = allowed, max 256 characters",
            location: "command::addkey::addkey",
        })
    } else {
        s.add_key(v)
    }
}
