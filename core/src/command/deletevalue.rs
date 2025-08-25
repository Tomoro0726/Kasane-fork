use crate::{
    command::tools::select::select,
    error::Error,
    json::{input::DeleteValue, output::Output},
};

#[cfg(feature = "full")]
use crate::io::kv::Storage;
#[cfg(feature = "wasm")]
use crate::io::memory::Storage;

pub fn deletevalue(v: DeleteValue, s: &mut Storage) -> Result<Output, Error> {
    let set = select(s, v.range)?;
    s.delete_value(&v.spacename, &v.keyname, set)
}
