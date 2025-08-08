use crate::{
    command::tools::select::select, error::Error, io::Storage, output::Output, parser::SetValue,
};

pub fn setvalue(v: SetValue, s: &mut Storage) -> Result<Output, Error> {
    let set = select(s, v.select)?;

    let space = s.get_space(&v.spacename)?;
    let key = space.get_key(&v.keyname)?;

    key.set_value(set, v.value)
}
