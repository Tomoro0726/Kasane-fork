use crate::{
    command::tools::valid_name::valid_name,
    error::Error,
    json::{input::AddSpace, output::Output},
};

#[cfg(feature = "wasm")]
use crate::io::memory::Storage;
#[cfg(feature = "full")]
use crate::io::sled::Storage;

pub fn addspace(v: AddSpace, s: &Storage) -> Result<Output, Error> {
    if !valid_name(&v.spacename) {
        Err(Error::SpaceNameValidationError {
            name: v.spacename,
            reason: "only a-z, A-Z, 0-9, - _ . @ + = allowed, max 256 characters",
            location: "command::addspace::addspace",
        })
    } else {
        s.add_space(v)
    }
}
