use crate::{
    command::tools::valid_name::valid_name, error::Error, io::Storage, output::Output,
    parser::DeleteKey,
};

pub fn deletekey(v: DeleteKey, s: &mut Storage) -> Result<Output, Error> {
    if !valid_name(&v.name) {
        Err(Error::KeyNameValidationError {
            name: v.name.clone(),
            reason: "only a-z, A-Z, 0-9, - _ . @ + = allowed, max 256 characters",
            location: "command::deletekey::deletekey",
        })
    } else {
        let space = s.get_space(&v.spacename)?;
        space.delete_key(&v.name)
    }
}
