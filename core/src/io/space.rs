use serde_json::Value;

use crate::error::Error;

use crate::io::Key;
use crate::io::output::IoOutput;
use crate::output::Output;
use crate::{
    io::{Space, error::IoError},
    parser::KeyType,
};

impl Space {
    pub fn add_key(&mut self, name: &str, r#type: KeyType) -> Result<Output, Error> {
        match self.get_key(name) {
            Ok(_) => Err(Error::IoError(IoError::KeyNameAlreadyExists("もうあるよ"))),
            Err(_) => {
                self.key.push(Key {
                    name: name.to_string(),
                    r#type,
                    value: Vec::new(),
                });
                Ok(Output::IoResult(IoOutput::Success))
            }
        }
    }

    pub fn delete_key(&mut self, name: &str) -> Result<Output, Error> {
        match self.get_key(name) {
            Ok(_) => {
                self.key.retain(|v| v.name != name);
                Ok(Output::IoResult(IoOutput::Success))
            }
            Err(e) => Err(e),
        }
    }

    pub fn get_key(&mut self, name: &str) -> Result<&mut Key, Error> {
        self.key
            .iter_mut()
            .find(|v| v.name == name)
            .ok_or(Error::IoError(IoError::KeyNameNotFound("Keyがないです。")))
    }

    pub fn show_keys(&self) -> Output {
        let mut result = Vec::new();
        for v in &self.key {
            result.push(v.name.clone());
        }
        Output::IoResult(IoOutput::KeyNames(result))
    }
}
