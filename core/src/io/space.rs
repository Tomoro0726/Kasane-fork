use std::io::Error;

use crate::{io::Space, parser::KeyType};

impl Space {
    fn add_key(&mut self, name: String, r#type: KeyType) -> Result<(), Error> {}
    fn delete_key(&mut self, name: String) {}
    fn find_key(self, name: String) {}
}
