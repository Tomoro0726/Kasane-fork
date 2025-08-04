use crate::{
    command::{error::CommandError, tools::valid_name::valid_name},
    error::Error,
    io::Storage,
    parser::AddSpace,
};

pub fn addspace(v: AddSpace, s: &mut Storage) -> Result<(), Error> {
    if !valid_name(&v.spacename) {
        Err(Error::CommandError(CommandError::SpaceNameValidationError(
            "Invalid name: only a-z, A-Z, 0-9, - _ . @ + = allowed, max 256 characters",
        )))
    } else {
        s.add_space(v.spacename)
    }
}
