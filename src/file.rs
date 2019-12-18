use crate::config::Aliases;
use crate::error::prelude::*;
use std::convert::TryFrom;
use std::path::PathBuf;

const ALIAS_PREFIX: &str = "@";

#[derive(Deserialize, Clone, Debug)]
#[serde(try_from = "&str")]
pub enum File {
    Files(Vec<PathBuf>),
    Alias(String),
}

impl TryFrom<&str> for File {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self> {
        if s.starts_with(ALIAS_PREFIX) {
            Ok(File::Alias(s.to_string()))
        } else {
            let mut files = Vec::new();
            // let path = PathBuf::from(s);
            for entry in glob::glob(s)
                .map_err(|e| Error::GlobError(s.to_string(), e.to_string()))?
            {
                let path = entry.map_err(|e| {
                    Error::GlobError(s.to_string(), e.to_string())
                })?;
                if path.exists() {
                    files.push(path);
                } else {
                    return Err(Error::FileNotFound(
                        path.as_os_str().to_str().unwrap().to_string(),
                    ));
                }
            }

            Ok(File::Files(files))
        }
    }
}

pub fn expand_files<'a>(
    file_list: &'a Vec<File>,
    aliases: &'a Aliases,
) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for file_or_alias in file_list.iter() {
        match &file_or_alias {
            File::Files(paths) => files.append(&mut paths.clone()),
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
