#[derive(Debug)]
pub enum Error {
    // Name validation errors with context
    SpaceNameValidationError {
        name: String,
        reason: &'static str,
        location: &'static str,
    },
    KeyNameValidationError {
        name: String,
        reason: &'static str,
        location: &'static str,
    },

    // Parse errors with context
    ParseError {
        message: String,
        location: &'static str,
    },

    // Resource existence errors
    SpaceAlreadyExists {
        space_name: String,
        location: &'static str,
    },
    SpaceNotFound {
        space_name: String,
        location: &'static str,
    },
    KeyNotFound {
        key_name: String,
        space_name: String,
        location: &'static str,
    },
    KeyAlreadyExists {
        key_name: String,
        space_name: String,
        location: &'static str,
    },

    // Value operation errors
    ValueAlreadyExists {
        space_time_id: String,
        location: &'static str,
    },
    TypeMismatchFilter {
        expected_type: String,
        operation: String,
        location: &'static str,
    },
    TypeMismatchValue {
        expected_type: String,
        received_type: String,
        location: &'static str,
    },
    // sled エラーをラップ
    StorageError {
        source: sled::Error,
        location: &'static str,
    },
}

use std::fmt;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::SpaceNameValidationError {
                name,
                reason,
                location,
            } => {
                write!(
                    f,
                    "Invalid space name '{}': {} (at {})",
                    name, reason, location
                )
            }
            Error::KeyNameValidationError {
                name,
                reason,
                location,
            } => {
                write!(
                    f,
                    "Invalid key name '{}': {} (at {})",
                    name, reason, location
                )
            }
            Error::ParseError { message, location } => {
                write!(f, "Parse error: {} (at {})", message, location)
            }
            Error::SpaceAlreadyExists {
                space_name,
                location,
            } => {
                write!(f, "Space '{}' already exists (at {})", space_name, location)
            }
            Error::SpaceNotFound {
                space_name,
                location,
            } => {
                write!(f, "Space '{}' not found (at {})", space_name, location)
            }
            Error::KeyNotFound {
                key_name,
                space_name,
                location,
            } => {
                write!(
                    f,
                    "Key '{}' not found in space '{}' (at {})",
                    key_name, space_name, location
                )
            }
            Error::KeyAlreadyExists {
                key_name,
                space_name,
                location,
            } => {
                write!(
                    f,
                    "Key '{}' already exists in space '{}' (at {})",
                    key_name, space_name, location
                )
            }
            Error::ValueAlreadyExists {
                space_time_id,
                location,
            } => {
                write!(
                    f,
                    "Value already exists for SpaceTimeId '{}' (at {})",
                    space_time_id, location
                )
            }
            Error::TypeMismatchFilter {
                expected_type,
                operation,
                location,
            } => {
                write!(
                    f,
                    "Type mismatch: expected '{}' type for {} operation (at {})",
                    expected_type, operation, location
                )
            }
            Error::TypeMismatchValue {
                expected_type,
                received_type,
                location,
            } => {
                write!(
                    f,
                    "Type mismatch: expected '{}' but received '{}' (at {})",
                    expected_type, received_type, location
                )
            }
            Error::StorageError { source, location } => {
                write!(f, "Storage error: {} (at {})", source, location)
            }
        }
    }
}
