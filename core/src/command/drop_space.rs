use std::sync::Arc;

use crate::{
    error::Error,
    io::{Storage, StorageTrait},
    json::{input::DropSpace, output::Output},
};

pub fn drop_space(v: DropSpace, s: Arc<Storage>) -> Result<Output, Error> {
    s.delete_space(&v.space_name)
}
