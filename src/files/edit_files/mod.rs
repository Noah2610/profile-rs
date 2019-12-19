pub mod prelude {
    pub use super::edit_files;
    pub use super::settings::EditSettings;
    pub use super::settings::EditSettingsBuilder;
}

mod settings;

use crate::context::Verbosity;
use crate::error::prelude::*;
use settings::EditSettings;
use std::path::PathBuf;

pub fn edit_files<'a>(
    files: &'a Vec<PathBuf>,
    settings: EditSettings,
) -> Result<()> {
    for file_path in files.iter() {
        let file_name = file_path.to_str().ok_or("PathBuf to string failed")?;

        if file_path.is_file() {
            settings.verbosity.print_at(
                Verbosity::Info,
                format!("Checking file '{}'", file_name),
            );
            edit_file(file_path, &settings)?;
        } else {
            return Err(Error::FileNotFound(file_name.to_string()));
        }
    }

    Ok(())
}

fn edit_file<'a>(
    file_path: &'a PathBuf,
    settings: &'a EditSettings,
) -> Result<()> {
    Ok(())
}
