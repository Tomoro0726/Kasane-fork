use std::sync::Arc;

use crate::{
    command::tools::select::select,
    error::Error,
    io::Storage,
    json::{input::PutValue, output::Output},
};

pub fn putvalue(v: PutValue, s: Arc<Storage>) -> Result<Output, Error> {
    let set = select(s, v.range)?;
    s.put_value(&v.spacename, &v.keyname, v.value, set)
}
