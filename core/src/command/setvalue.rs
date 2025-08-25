use crate::{
    command::tools::select::select,
    error::Error,
    io::Storage,
    json::{input::SetValue, output::Output},
};

pub fn setvalue(v: SetValue, s: &mut Storage) -> Result<Output, Error> {
    let set = select(s, v.range)?;

    let space = s.get_space(&v.spacename)?;
    let key = space.get_key(&v.keyname)?;

    key.set_value(set, v.value)
}
