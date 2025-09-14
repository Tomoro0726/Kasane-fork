use std::sync::Arc;

use crate::{
    error::Error,
    io::{Storage, StorageTrait},
    json::{input::Command, output::Output},
};

pub fn transaction(v: Vec<Command>, s: Arc<Storage>) -> Result<Output, Error> {}
