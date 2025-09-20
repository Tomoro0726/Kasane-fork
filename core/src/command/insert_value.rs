use std::sync::Arc;

use crate::{
    error::Error,
    io::full::Storage,
    json::{input::InsertValue, output::Output},
};

pub fn insert_value(v: InsertValue, s: Arc<Storage>) -> Result<Output, Error> {
    todo!()
}
