use crate::config::Aliases;
use crate::error::prelude::*;
use crate::ALIAS_PREFIX;
use std::convert::TryFrom;
use std::path::PathBuf;

#[derive(Deserialize, Clone, Debug)]
// #[serde(try_from = "&str")]
#[serde(try_from = "Vec<&str>")]
pub enum FileList {
    File(PathBuf),
    Files(Vec<Box<FileList>>),
    Dir(PathBuf),
    Alias(String),
}

impl FileList {
    pub fn is_empty(&self) -> bool {
        if let FileList::Files(files) = self {
            files.is_empty()
        } else {
            false
        }
    }
}

impl TryFrom<Vec<&str>> for FileList {
    type Error = Error;

    fn try_from(v: Vec<&str>) -> Result<Self> {
        let mut files = Vec::new();
        for s in v {
            files.push(Box::new(Self::try_from(s)?));
        }
        Ok(FileList::Files(files))
    }
}

impl TryFrom<&str> for FileList {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self> {
        let s = if let Some(home_dir) = dirs::home_dir() {
            s.replace("~", home_dir.as_os_str().to_str().unwrap())
        } else {
            s.to_string()
        };

        if s.starts_with(ALIAS_PREFIX) {
            Ok(FileList::Alias(s.to_string()))
        } else {
            let mut files = Vec::new();
            for entry in glob::glob(s.as_str())
                .map_err(|e| Error::GlobError(s.to_string(), e.to_string()))?
            {
                let path = entry.map_err(|e| {
                    Error::GlobError(s.to_string(), e.to_string())
                })?;
                if path.exists() {
                    if path.is_file() {
                        files.push(Box::new(FileList::File(path)));
                    } else if path.is_dir() {
                        files.push(Box::new(FileList::Dir(path)));
                    }
                } else {
                    return Err(Error::FileNotFound(
                        path.as_os_str().to_str().unwrap().to_string(),
                    ));
                }
            }

            if files.is_empty() {
                Err(Error::FileNotFound(s.to_string()))
            } else {
                Ok(FileList::Files(files))
            }
        }
    }
}

impl Into<Vec<FileList>> for FileList {
    fn into(self) -> Vec<FileList> {
        vec![self]
    }
}

impl From<Vec<FileList>> for FileList {
    fn from(files: Vec<FileList>) -> Self {
        FileList::Files(files.into_iter().map(|f| Box::new(f)).collect())
    }
}

pub fn expand_files<'a>(
    file_list: &'a FileList,
    aliases: &'a Aliases,
) -> Result<Vec<PathBuf>> {
    let mut file_paths = Vec::new();

    match &file_list {
        FileList::File(path) => file_paths.push(path.clone()),
        FileList::Files(files) => {
            for file in files {
                file_paths.append(&mut expand_files(&*file, aliases)?)
            }
        }
        FileList::Dir(dir_path) => {
            for file_path_res in dir_path.read_dir().map_err(|e| {
                Error::FsReadError(
                    dir_path.as_os_str().to_str().unwrap().to_string(),
                    e.to_string(),
                )
            })? {
                let file_path = file_path_res
                    .map_err(|e| {
                        Error::FsReadError(
                            dir_path.as_os_str().to_str().unwrap().to_string(),
                            e.to_string(),
                        )
                    })?
                    .path();
                if file_path.is_file() {
                    file_paths.push(file_path)
                } else {
                    // TODO recurse if option is set.
                }
            }
        }
        FileList::Alias(alias) => file_paths.append(&mut expand_files(
            aliases
                .get(&alias)
                .ok_or(Error::AliasNotFound(alias.to_string()))?,
            aliases,
        )?),
    }

    Ok(file_paths)
}
