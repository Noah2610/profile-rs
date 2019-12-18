use crate::file_list::FileList;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct FilesConfig {
    pub aliases: Aliases,
}

#[derive(Deserialize, Debug)]
pub struct Aliases(HashMap<String, FileList>);

impl Aliases {
    pub fn get(&self, name: &str) -> Option<&FileList> {
        self.0.get(name)
    }
}
