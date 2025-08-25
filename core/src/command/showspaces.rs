use crate::{error::Error, io::Storage, json::output::Output};

pub fn showspaces(s: &mut Storage) -> Result<Output, Error> {
    Ok(s.show_spaces())
}
