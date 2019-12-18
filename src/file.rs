use crate::config::Aliases;
use crate::error::prelude::*;
use std::convert::TryFrom;
use std::path::PathBuf;

const ALIAS_PREFIX: &str = "@";

#[derive(Deserialize, Clone, Debug)]
// #[serde(try_from = "&str")]
#[serde(try_from = "Vec<&str>")]
pub enum File {
    File(PathBuf),
    Files(Vec<Box<File>>),
    Alias(String),
}

impl TryFrom<Vec<&str>> for File {
    type Error = Error;

    fn try_from(v: Vec<&str>) -> Result<Self> {
        let mut files = Vec::new();
        for s in v {
            files.push(Box::new(Self::try_from(s)?));
        }
        Ok(File::Files(files))
    }
}

impl TryFrom<&str> for File {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self> {
        let s = if let Some(home_dir) = dirs::home_dir() {
            s.replace("~", home_dir.as_os_str().to_str().unwrap())
        } else {
            s.to_string()
        };

        if s.starts_with(ALIAS_PREFIX) {
            Ok(File::Alias(s.to_string()))
        } else {
            let mut files = Vec::new();
            for entry in glob::glob(s.as_str())
                .map_err(|e| Error::GlobError(s.to_string(), e.to_string()))?
            {
                let path = entry.map_err(|e| {
                    Error::GlobError(s.to_string(), e.to_string())
                })?;
                if path.exists() {
                    files.push(Box::new(File::File(path)));
                } else {
                    return Err(Error::FileNotFound(
                        path.as_os_str().to_str().unwrap().to_string(),
                    ));
                }
            }

            if files.is_empty() {
                Err(Error::FileNotFound(s.to_string()))
            } else {
                Ok(File::Files(files))
            }
        }
    }
}

impl Into<Vec<File>> for File {
    fn into(self) -> Vec<File> {
        vec![self]
    }
}

impl From<Vec<File>> for File {
    fn from(files: Vec<File>) -> Self {
        File::Files(files.into_iter().map(|f| Box::new(f)).collect())
    }
}

pub fn expand_files<'a>(
    file_list: &'a File,
    aliases: &'a Aliases,
) -> Result<Vec<PathBuf>> {
    let mut file_paths = Vec::new();

    match &file_list {
        File::File(path) => file_paths.push(path.clone()),
        File::Files(files) => {
            for file in files {
                file_paths.append(&mut expand_files(&*file, aliases)?)
            }
        }
        File::Alias(alias) => file_paths.append(&mut expand_files(
            aliases
                .get(&alias)
                .ok_or(Error::AliasNotFound(alias.to_string()))?,
            aliases,
        )?),
    }

    Ok(file_paths)
}
