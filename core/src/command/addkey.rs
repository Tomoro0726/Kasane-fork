use std::sync::Arc;

use crate::io::{Storage, StorageTrait};
use crate::json::input::AddKey;
use crate::json::output::Output;
use crate::{command::tools::valid_name::valid_name, error::Error};

pub fn addkey(v: AddKey, s: Arc<Storage>) -> Result<Output, Error> {
    if !valid_name(&v.keyname) {
        Err(Error::KeyNameValidationError {
            name: v.keyname,
            reason: "only a-z, A-Z, 0-9, - _ . @ + = allowed, max 256 characters",
            location: "command::addkey::addkey",
        })
    } else {
        s.add_key(&v.spacename, &v.keyname, v.keytype)
    }
}
