pub enum IoError {
    SpaceNameAlreadyExists(&'static str),
    SpaceNameNotFound(&'static str),
}
