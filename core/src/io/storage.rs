use crate::{
    error::Error,
    io::{Space, Storage},
    output::Output,
};

impl Storage {
    pub fn add_space(&mut self, name: &str) -> Result<Output, Error> {
        match self.get_space(name) {
            Ok(_) => {
                return Err(Error::SpaceAlreadyExists {
                    space_name: name.to_string(),
                    location: "io::storage::add_space",
                });
            }
            Err(_) => {
                self.space.push(Space {
                    name: name.to_string(),
                    key: Vec::new(),
                });
                Ok(Output::Success)
            }
        }
    }
    pub fn delete_space(&mut self, name: &str) -> Result<Output, Error> {
        match self.get_space(name) {
            Ok(_) => {
                self.space.retain(|v| v.name != name);
                Ok(Output::Success)
            }
            Err(e) => Err(e),
        }
    }
    pub fn get_space(&mut self, name: &str) -> Result<&mut Space, Error> {
        self.space
            .iter_mut()
            .find(|v| v.name == name)
            .ok_or(Error::SpaceNotFound {
                space_name: name.to_string(),
                location: "io::storage::get_space",
            })
    }
    pub fn show_spaces(&self) -> Output {
        let mut result = Vec::new();
        for v in &self.space {
            result.push(v.name.clone());
        }
        Output::SpaceNames(result)
    }
}
