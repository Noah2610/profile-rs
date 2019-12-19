pub mod prelude {
    pub use super::edit_files;
    pub use super::settings::EditSettings;
    pub use super::settings::EditSettingsBuilder;
}

mod settings;

use crate::error::prelude::*;
use settings::EditSettings;
use std::path::PathBuf;

pub fn edit_files<'a>(
    files: &'a Vec<PathBuf>,
    settings: EditSettings,
) -> Result<()> {
    for file_path in files.iter() {
        if file_path.is_file() {

        } else {
            return Err(Error::FileNotFound(
                file_path.as_os_str().to_str().unwrap().to_string(),
            ));
        }
    }

    Ok(())
}
