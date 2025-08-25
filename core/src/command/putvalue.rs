use crate::{
    command::tools::select::select,
    error::Error,
    json::{input::PutValue, output::Output},
};

#[cfg(feature = "full")]
use crate::io::kv::Storage;
#[cfg(feature = "wasm")]
use crate::io::memory::Storage;

pub fn putvalue(v: PutValue, s: &mut Storage) -> Result<Output, Error> {
    let set = select(s, v.range)?;
    s.put_value(&v.spacename, &v.keyname, v.value, set)
}
