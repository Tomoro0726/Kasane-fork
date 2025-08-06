use logic::set::SpaceTimeIdSet;

use crate::{
    error::Error,
    io::{Space, Storage, error::IoError, output::IoOutput},
    output::Output,
    parser::Select,
};

impl Storage {
    pub fn add_space(&mut self, name: &str) -> Result<Output, Error> {
        match self.get_space(name) {
            Ok(_) => {
                return Err(Error::IoError(IoError::SpaceNameAlreadyExists(
                    "Spaceがもう存在する",
                )));
            }
            Err(_) => {
                self.space.push(Space {
                    name: name.to_string(),
                    key: Vec::new(),
                });
                Ok(Output::IoResult(IoOutput::Success))
            }
        }
    }
    pub fn delete_space(&mut self, name: &str) -> Result<Output, Error> {
        match self.get_space(name) {
            Ok(_) => {
                self.space.retain(|v| v.name != name);
                Ok(Output::IoResult(IoOutput::Success))
            }
            Err(e) => Err(e),
        }
    }
    pub fn get_space(&mut self, name: &str) -> Result<&mut Space, Error> {
        self.space
            .iter_mut()
            .find(|v| v.name == name)
            .ok_or(Error::IoError(IoError::SpaceNameNotFound("Spaceがない")))
    }
    pub fn show_spaces(&self) -> Output {
        let mut result = Vec::new();
        for v in &self.space {
            result.push(v.name.clone());
        }
        Output::IoResult(IoOutput::SpaceNames(result))
    }
}
