use std::sync::Arc;

use crate::{
    command::tools::select::select,
    error::Error,
    json::{input::DeleteValue, output::Output},
};

use crate::io::Storage;

pub fn deletevalue(v: DeleteValue, s: Arc<Storage>) -> Result<Output, Error> {
    let set = select(s, v.range)?;
    s.delete_value(&v.spacename, &v.keyname, set)
}
