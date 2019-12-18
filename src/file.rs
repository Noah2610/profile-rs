use crate::config::Aliases;
use crate::error::prelude::*;
use std::convert::TryFrom;
use std::path::PathBuf;

const ALIAS_PREFIX: &str = "@";

#[derive(Deserialize, Clone, Debug)]
#[serde(try_from = "&str")]
pub enum File {
    File(PathBuf),
    Alias(String),
}

impl TryFrom<&str> for File {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self> {
        if s.starts_with(ALIAS_PREFIX) {
            Ok(File::Alias(s.to_string()))
        } else {
            let path = PathBuf::from(s);
            if path.exists() {
                Ok(File::File(path))
            } else {
                Err(Error::FileNotFound(s.to_string()))
            }
        }
    }
}

pub fn expand_files<'a>(
    file_list: &'a Vec<File>,
    aliases: &'a Aliases,
) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for file_or_alias in file_list.into_iter() {
        match file_or_alias {
            File::File(path) => files.push(path.clone()),
            File::Alias(alias) => files.append(&mut expand_files(
                aliases
                    .get(&alias)
                    .ok_or(Error::AliasNotFound(alias.to_string()))?,
                &aliases,
            )?),
        }
    }

    Ok(files)
}
