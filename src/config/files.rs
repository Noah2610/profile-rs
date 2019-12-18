use crate::file::File;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct FilesConfig {
    pub aliases: Aliases,
}

#[derive(Deserialize, Debug)]
pub struct Aliases(HashMap<String, Vec<File>>);

impl Aliases {
    pub fn get(&self, name: &str) -> Option<&Vec<File>> {
        self.0.get(name)
    }
}
