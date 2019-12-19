pub mod prelude {
    pub use super::expand_files;
    pub use super::settings::ExpandSettings;
    pub use super::settings::ExpandSettingsBuilder;
}

mod settings;

use super::FileList;
use crate::config::Aliases;
use crate::context::Verbosity;
use crate::error::prelude::*;
use settings::ExpandSettings;
use std::path::PathBuf;

pub fn expand_files<'a>(
    file_list: &'a FileList,
    aliases: &'a Aliases,
    settings: &'a ExpandSettings,
) -> Result<Vec<PathBuf>> {
    let mut file_paths = FilePaths::builder()
        .verbosity(settings.verbosity)
        .build()
        .unwrap();

    match file_list {
        FileList::File(path) => file_paths.push(path.clone()),

        FileList::Files(files) => {
            for file in files {
                file_paths.append(expand_files(&*file, aliases, settings)?)
            }
        }

        FileList::Dir(dir_path) => {
            let dir_name = dir_path.as_os_str().to_str().unwrap();
            settings.verbosity.print_at(
                Verbosity::Debug,
                format!("Including directory '{}'", dir_name),
            );

            for file_path_res in dir_path.read_dir().map_err(|e| {
                Error::FsReadError(dir_name.to_string(), e.to_string())
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
                    file_paths.push(file_path);
                } else if file_path.is_dir() && settings.recurse {
                    file_paths.append(expand_files(
                        &FileList::Dir(file_path),
                        aliases,
                        settings,
                    )?)
                }
            }
        }

        FileList::Alias(alias) => file_paths.append(expand_files(
            aliases
                .get(&alias)
                .ok_or(Error::AliasNotFound(alias.to_string()))?,
            aliases,
            settings,
        )?),

        FileList::Empty => (),
    }

    Ok(file_paths.into())
}

#[derive(Default, Builder)]
#[builder(pattern = "owned", setter(into))]
struct FilePaths {
    #[builder(setter(skip), default)]
    paths: Vec<PathBuf>,
    #[builder(default)]
    verbosity: Verbosity,
}

impl FilePaths {
    pub fn builder() -> FilePathsBuilder {
        FilePathsBuilder::default()
    }

    pub fn push<P: Into<PathBuf>>(&mut self, path: P) {
        let path = path.into();
        self.verbosity.print_at(
            Verbosity::Debug,
            format!("Including file '{}'", path.as_os_str().to_str().unwrap()),
        );
        self.paths.push(path);
    }

    pub fn append<V, P>(&mut self, paths: V)
    where
        V: Into<Vec<P>>,
        P: Into<PathBuf>,
    {
        self.paths
            .append(&mut paths.into().into_iter().map(Into::into).collect());
    }
}

impl Into<Vec<PathBuf>> for FilePaths {
    fn into(self) -> Vec<PathBuf> {
        self.paths
    }
}
