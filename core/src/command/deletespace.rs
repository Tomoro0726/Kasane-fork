use crate::{error::Error, io::Storage, output::Output, parser::DeleteSpace};

pub fn deletespace(v: DeleteSpace, s: &mut Storage) -> Result<Output, Error> {
    s.delete_space(&v.spacename)
}
