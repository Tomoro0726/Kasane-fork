#[derive(Debug)]
pub enum Error {
    SpaceNameValidationError(&'static str),
    KeyNameValidationError(&'static str),
    ParseError(String),
    SpaceNameAlreadyExists(&'static str),
    SpaceNameNotFound(&'static str),
    KeyNameNotFound(&'static str),
    KeyNameAlreadyExists(&'static str),
    SpaceTimeIdAlreadyHasValue(&'static str),
}

use std::fmt;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::SpaceNameValidationError(name) => write!(f, "Invalid space name: {}", name),
            Error::KeyNameValidationError(name) => write!(f, "Invalid key name: {}", name),
            Error::ParseError(err) => write!(f, "Parse error: {}", err),
            Error::SpaceNameAlreadyExists(name) => write!(f, "Space name already exists: {}", name),
            Error::SpaceNameNotFound(name) => write!(f, "Space name not found: {}", name),
            Error::KeyNameNotFound(name) => write!(f, "Key name not found: {}", name),
            Error::KeyNameAlreadyExists(name) => write!(f, "Key name already exists: {}", name),
            Error::SpaceTimeIdAlreadyHasValue(id) => {
                write!(f, "SpaceTimeId already has value: {}", id)
            }
        }
    }
}
