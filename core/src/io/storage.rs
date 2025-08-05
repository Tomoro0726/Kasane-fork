use crate::{
    error::Error,
    io::{Space, Storage, error::IoError},
};

impl Storage {
    pub fn add_space(&mut self, name: String) -> Result<(), Error> {
        match self.find_space(&name) {
            Some(_) => Err(Error::IoError(IoError::SpaceNameAlreadyExists(
                "Space name already exists",
            ))),
            None => {
                self.space.push(Space {
                    name,
                    key: Vec::new(),
                });
                Ok(())
            }
        }
    }
    pub fn delete_space(&mut self, name: String) -> Result<(), Error> {
        match self.find_space(&name) {
            Some(space) => {
                space.key.retain(|v| v.name != name);
                Ok(())
            }
            None => Err(Error::IoError(IoError::SpaceNameNotFound(
                "Space name not found",
            ))),
        }
    }
    pub fn find_space(&mut self, name: &str) -> Option<&mut Space> {
        self.space.iter_mut().find(|v| v.name == name)
    }
}
