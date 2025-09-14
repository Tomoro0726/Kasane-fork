use std::sync::Arc;

use crate::{
    command::tools::valid_name::valid_name,
    error::Error,
    io::{Storage, StorageTrait},
    json::{input::CreateSpace, output::Output},
};

pub fn addspace(v: CreateSpace, s: Arc<Storage>) -> Result<Output, Error> {
    if !valid_name(&v.space_name) {
        Err(Error::SpaceNameValidationError {
            name: v.space_name,
            reason: "only a-z, A-Z, 0-9, - _ . @ + = allowed, max 256 characters",
            location: "command::addspace::addspace",
        })
    } else {
        s.add_space(&v.space_name)
    }
}
