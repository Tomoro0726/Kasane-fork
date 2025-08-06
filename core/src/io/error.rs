pub enum IoError {
    SpaceNameAlreadyExists(&'static str),
    SpaceNameNotFound(&'static str),
    KeyNameNotFound(&'static str),
    KeyNameAlreadyExists(&'static str),
    SpaceTimeIdAlreadyHasValue(&'static str),
}
