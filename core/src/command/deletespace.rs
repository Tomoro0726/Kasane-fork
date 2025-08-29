use std::sync::Arc;

use crate::{
    error::Error,
    io::{Storage, StorageTrait},
    json::{input::DeleteSpace, output::Output},
};

pub fn deletespace(v: DeleteSpace, s: Arc<Storage>) -> Result<Output, Error> {
    s.delete_space(&v.spacename)
}
