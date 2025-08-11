use crate::{error::Error, io::Storage, output::Output, parser::Keys};

pub fn showkeys(v: Keys, s: &mut Storage) -> Result<Output, Error> {
    let space = s.get_space(&v.spacename)?;
    Ok(space.show_keys())
}
