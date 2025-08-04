use crate::{error::Error, io::Storage, parser::DeleteSpace};

pub fn deletespace(v: DeleteSpace, s: &mut Storage) -> Result<(), Error> {
    s.delete_space(v.spacename)
}
