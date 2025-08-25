use crate::{
    error::Error,
    io::Storage,
    json::{input::Keys, output::Output},
};

pub fn showkeys(v: Keys, s: &mut Storage) -> Result<Output, Error> {
    let space = s.get_space(&v.spacename)?;
    Ok(space.show_keys())
}
