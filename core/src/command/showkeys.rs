use crate::{error::Error, io::Storage, output::Output, parser::keys};

pub fn showkeys(v: keys, s: &mut Storage) -> Result<Output, Error> {
    let space = s.get_space(&v.spacename)?;
    Ok(space.show_keys())
}
