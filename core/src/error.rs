use crate::{command::error::CommandError, io::error::IoError};

pub enum Error {
    IoError(IoError),
    CommandError(CommandError),
}
