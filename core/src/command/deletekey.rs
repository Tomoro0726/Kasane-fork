use crate::{
    command::tools::valid_name::valid_name,
    error::Error,
    json::{input::DeleteKey, output::Output},
};

#[cfg(feature = "full")]
use crate::io::kv::Storage;
#[cfg(feature = "wasm")]
use crate::io::memory::Storage;

pub fn deletekey(v: DeleteKey, s: &mut Storage) -> Result<Output, Error> {
    if !valid_name(&v.keyname) {
        Err(Error::KeyNameValidationError {
            name: v.keyname.clone(),
            reason: "only a-z, A-Z, 0-9, - _ . @ + = allowed, max 256 characters",
            location: "command::deletekey::deletekey",
        })
    } else {
        s.delete_key(v)
    }
}
