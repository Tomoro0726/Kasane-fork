use std::sync::Arc;

use crate::{
    command::tools::select::select,
    error::Error,
    io::Storage,
    json::{input::SetValue, output::Output},
};

pub fn setvalue(v: SetValue, s: Arc<Storage>) -> Result<Output, Error> {
    let set = select(s, v.range)?;
    s.set_value(&v.spacename, &v.keyname, v.value, set)
}
