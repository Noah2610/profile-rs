mod settings;

use super::FileList;
use crate::config::Aliases;
use crate::error::prelude::*;
use std::path::PathBuf;

pub use settings::ExpandSettings;
pub use settings::ExpandSettingsBuilder;

pub fn expand_files<'a>(
    file_list: &'a FileList,
    aliases: &'a Aliases,
    settings: &'a ExpandSettings,
) -> Result<Vec<PathBuf>> {
    let mut file_paths = Vec::new();

    match &file_list {
        FileList::File(path) => file_paths.push(path.clone()),

        FileList::Files(files) => {
            for file in files {
                file_paths.append(&mut expand_files(&*file, aliases, settings)?)
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
                } else if file_path.is_dir() && settings.recurse {
                    file_paths.append(&mut expand_files(
                        &FileList::Dir(file_path),
                        aliases,
                        settings,
                    )?)
                }
            }
        }

        FileList::Alias(alias) => file_paths.append(&mut expand_files(
            aliases
                .get(&alias)
                .ok_or(Error::AliasNotFound(alias.to_string()))?,
            aliases,
            settings,
        )?),

        FileList::Empty => (),
    }

    Ok(file_paths)
}
