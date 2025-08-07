use crate::{
    command::tools::valid_name::valid_name, error::Error, io::Storage, output::Output,
    parser::AddKey,
};

pub fn addkey(v: AddKey, s: &mut Storage) -> Result<Output, Error> {
    if !valid_name(&v.spacename) {
        Err(Error::KeyNameValidationError(
            "Invalid name: only a-z, A-Z, 0-9, - _ . @ + = allowed, max 256 characters",
        ))
    } else {
        let space = s.get_space(&v.spacename)?;
        space.add_key(&v.name, v.r#type)
    }
}
