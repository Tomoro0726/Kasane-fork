use crate::{
    error::Error,
    io::Storage,
    json::{input::DeleteSpace, output::Output},
};

pub fn deletespace(v: DeleteSpace, s: &mut Storage) -> Result<Output, Error> {
    s.delete_space(&v.spacename)
}
