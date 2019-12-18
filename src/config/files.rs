use crate::file_list::FileList;
use crate::ALIAS_PREFIX;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct FilesConfig {
    pub default: FileList,
    pub aliases: Aliases,
}

#[derive(Deserialize, Debug)]
pub struct Aliases(HashMap<String, FileList>);

impl Aliases {
    pub fn get(&self, name: &str) -> Option<&FileList> {
        let name = if name.starts_with(ALIAS_PREFIX) {
            name.replacen(ALIAS_PREFIX, "", 1)
        } else {
            name.to_string()
        };
        self.0.get(&name)
    }
}
