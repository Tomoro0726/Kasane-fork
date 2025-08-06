use crate::{error::Error, io::Storage, output::Output, parser::Showkeys};

pub fn showkeys(v: Showkeys, s: &mut Storage) -> Result<Output, Error> {
    let space = s.get_space(&v.spacename)?;
    Ok(space.show_keys())
}
