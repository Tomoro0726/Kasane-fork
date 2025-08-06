#[derive(Debug)]
pub enum CommandError {
    SpaceNameValidationError(&'static str),
    KeyNameValidationError(&'static str),
    ParseError(String),
}
