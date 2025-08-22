use crate::{error::Error, io::Storage, output::Output, parser::KeysInfo};

pub fn keysinfo(v: KeysInfo, s: &mut Storage) -> Result<Output, Error> {
    let space = s.get_space(&v.spacename)?;
    Ok(space.info_keys())
}
