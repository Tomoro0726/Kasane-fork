use crate::error::Error;

use crate::io::Key;
use crate::output::{KeyInfoOutput, Output};
use crate::{io::Space, parser::KeyType};

impl Space {
    pub fn add_key(&mut self, name: &str, r#type: KeyType) -> Result<Output, Error> {
        match self.get_key(name) {
            Ok(_) => Err(Error::KeyAlreadyExists {
                key_name: name.to_string(),
                space_name: self.name.clone(),
                location: "io::space::add_key",
            }),
            Err(_) => {
                self.key.push(Key {
                    name: name.to_string(),
                    r#type,
                    value: Vec::new(),
                });
                Ok(Output::Success)
            }
        }
    }

    pub fn delete_key(&mut self, name: &str) -> Result<Output, Error> {
        match self.get_key(name) {
            Ok(_) => {
                self.key.retain(|v| v.name != name);
                Ok(Output::Success)
            }
            Err(e) => Err(e),
        }
    }

    pub fn get_key(&mut self, name: &str) -> Result<&mut Key, Error> {
        self.key
            .iter_mut()
            .find(|v| v.name == name)
            .ok_or(Error::KeyNotFound {
                key_name: name.to_string(),
                space_name: self.name.clone(),
                location: "io::space::get_key",
            })
    }

    pub fn show_keys(&self) -> Output {
        let mut result = Vec::new();
        for v in &self.key {
            result.push(v.name.clone());
        }
        Output::KeyNames(result)
    }

    pub fn info_keys(&self) -> Output {
        let mut result = Vec::new();
        for v in &self.key {
            result.push(KeyInfoOutput {
                keyname: v.name.clone(),
                keytype: v.r#type.clone(),
            });
        }
        Output::KeysInfo(result)
    }
}
