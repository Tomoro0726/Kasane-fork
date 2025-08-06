use crate::{
    command::tools::select::select, error::Error, io::Storage, output::Output, parser::PutValue,
};

pub fn putvalue(v: PutValue, s: &mut Storage) -> Result<Output, Error> {
    let space = s.get_space(&v.spacename)?;
    let key = space.get_key(&v.keyname)?;

    let set = select(v.select)?;

    key.put_value(set, v.value)
}
