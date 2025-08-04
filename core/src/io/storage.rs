use crate::{
    error::Error,
    io::{Space, Storage, error::IoError},
};

impl Storage {
    pub fn add_space(&mut self, name: String) -> Result<(), Error> {
        if self.find_space(&name) {
            self.space.push(Space {
                name,
                key: Vec::new(),
            });
            Ok(())
        } else {
            Err(Error::IoError(IoError::SpaceNameAlreadyExists(
                "Space name already exists",
            )))
        }
    }
    pub fn delete_space(&mut self, name: String) -> Result<(), Error> {
        if self.find_space(&name) {
            self.space.retain(|v| v.name != name);
            Ok(())
        } else {
            Err(Error::IoError(IoError::SpaceNameNotFound(
                "Space name not found",
            )))
        }
    }
    pub fn find_space(&self, name: &str) -> bool {
        self.space.iter().any(|v| v.name == name)
    }
}
