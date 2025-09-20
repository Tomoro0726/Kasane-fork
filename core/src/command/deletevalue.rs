use std::sync::Arc;

use crate::{
    error::Error,
    io::full::Storage,
    json::{input::DeleteValue, output::Output},
};

pub fn deletevalue(v: DeleteValue, s: Arc<Storage>) -> Result<Output, Error> {
    todo!()
}
