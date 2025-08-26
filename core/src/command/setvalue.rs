use crate::{
    command::tools::select::select,
    error::Error,
    json::{input::SetValue, output::Output},
};

#[cfg(feature = "full")]
use crate::io::sled::Storage;
#[cfg(feature = "wasm")]
use crate::io::memory::Storage;

pub fn setvalue(v: SetValue, s: & Storage) -> Result<Output, Error> {
    let set = select(s, v.range)?;
    s.set_value(&v.spacename, &v.keyname, v.value, set)
}
