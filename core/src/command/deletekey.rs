use crate::{
    command::{error::CommandError, tools::valid_name::valid_name},
    error::Error,
    io::Storage,
    output::Output,
    parser::DeleteKey,
};

pub fn deletekey(v: DeleteKey, s: &mut Storage) -> Result<Output, Error> {
    if !valid_name(&v.spacename) {
        Err(Error::CommandError(CommandError::KeyNameValidationError(
            "Invalid name: only a-z, A-Z, 0-9, - _ . @ + = allowed, max 256 characters",
        )))
    } else {
        let space = s.get_space(&v.spacename)?;
        space.delete_key(&v.name)
    }
}
