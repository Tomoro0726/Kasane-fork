use std::sync::Arc;

use crate::{
    error::Error,
    io::{Storage, StorageTrait},
    json::output::Output,
};

pub fn showspaces(s: Arc<Storage>) -> Result<Output, Error> {
    s.show_spaces()
}
