use crate::{
    error::Error,
    io::memory::{Space, Storage},
    json::{
        input::{AddSpace, DeleteSpace},
        output::Output,
    },
};

impl Storage {
    pub fn get_space(&mut self, name: &str) -> Result<&mut Space, Error> {
        self.space
            .iter_mut()
            .find(|v| v.name == name)
            .ok_or(Error::SpaceNotFound {
                space_name: name.to_string(),
                location: "io::storage::get_space",
            })
    }
    pub fn add_space(&mut self, v: AddSpace) -> Result<Output, Error> {
        match self.get_space(&v.spacename) {
            Ok(_) => {
                return Err(Error::SpaceAlreadyExists {
                    space_name: v.spacename.to_string(),
                    location: "io::storage::add_space",
                });
            }
            Err(_) => {
                self.space.push(Space {
                    name: v.spacename.to_string(),
                    key: Vec::new(),
                });
                Ok(Output::Success)
            }
        }
    }
    pub fn delete_space(&mut self, v: DeleteSpace) -> Result<Output, Error> {
        match self.get_space(&v.spacename) {
            Ok(_) => {
                self.space.retain(|a| a.name != v.spacename);
                Ok(Output::Success)
            }
            Err(e) => Err(e),
        }
    }

    pub fn show_spaces(&self) -> Result<Output, Error> {
        let mut result = Vec::new();
        for v in &self.space {
            result.push(v.name.clone());
        }
        Ok(Output::SpaceNames(result))
    }
}
