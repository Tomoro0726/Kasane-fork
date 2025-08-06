use crate::{command::error::CommandError, io::error::IoError};
#[derive(Debug)]
pub enum Error {
    IoError(IoError),
    CommandError(CommandError),
}
