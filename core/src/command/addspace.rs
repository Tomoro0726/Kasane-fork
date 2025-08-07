use crate::{
    command::tools::valid_name::valid_name, error::Error, io::Storage, output::Output,
    parser::AddSpace,
};

pub fn addspace(v: AddSpace, s: &mut Storage) -> Result<Output, Error> {
    if !valid_name(&v.spacename) {
        Err(Error::SpaceNameValidationError(
            "Invalid name: only a-z, A-Z, 0-9, - _ . @ + = allowed, max 256 characters",
        ))
    } else {
        s.add_space(&v.spacename)
    }
}
