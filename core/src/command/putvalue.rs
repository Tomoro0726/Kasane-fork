use crate::{
    command::tools::select::select,
    error::Error,
    io::Storage,
    json::{input::PutValue, output::Output},
};

pub fn putvalue(v: PutValue, s: &mut Storage) -> Result<Output, Error> {
    let set = select(s, v.range)?;

    let space = s.get_space(&v.spacename)?;
    let key = space.get_key(&v.keyname)?;

    key.put_value(set, v.value)
}
