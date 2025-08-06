use crate::{error::Error, io::Storage, output::Output, parser::Showkeys};

pub fn showspaces(s: &mut Storage) -> Result<Output, Error> {
    Ok(s.show_spaces())
}
