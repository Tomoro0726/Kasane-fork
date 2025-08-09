use crate::{
    command::tools::select::select, error::Error, io::Storage, output::Output, parser::GetValue,
};

pub fn getvalue(v: GetValue, s: &mut Storage) -> Result<Output, Error> {
    let set = select(s, v.range)?;

    let space = s.get_space(&v.spacename)?;
    let key = space.get_key(&v.keyname)?;
    key.get_value(set)
}
