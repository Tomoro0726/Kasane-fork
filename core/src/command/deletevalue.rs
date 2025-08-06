use crate::{
    command::tools::select::select, error::Error, io::Storage, output::Output, parser::DeleteValue,
};

pub fn deletevalue(v: DeleteValue, s: &mut Storage) -> Result<Output, Error> {
    let space = s.get_space(&v.spacename)?;
    let key = space.get_key(&v.keyname)?;

    let set = select(v.select)?;

    key.delete_value(set)
}
