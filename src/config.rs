use crate::error::prelude::*;
use crate::meta;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct Config {}

impl Config {
    pub fn new() -> Result<Self> {
        let config_path = config_path().ok_or(Error::NoConfig)?;
        let config_path_str = config_path.to_str().unwrap();

        let mut config_file = File::open(&config_path).map_err(|io_err| {
            Error::ConfigReadError(
                config_path_str.to_string(),
                io_err.to_string(),
            )
        })?;

        let mut file_content = String::new();
        config_file.read_to_string(&mut file_content).unwrap();

        toml::de::from_str(file_content.as_str()).map_err(|toml_err| {
            Error::ConfigParseError(
                config_path_str.to_string(),
                toml_err.to_string(),
            )
        })
    }
}

fn config_path() -> Option<PathBuf> {
    let mut paths = Vec::new();
    paths.push(PathBuf::from("./config.toml"));
    if let Some(mut home_dir) = dirs::home_dir() {
        home_dir.push(PathBuf::from(&format!(".{}.toml", meta::NAME)));
        paths.push(home_dir);
    }
    if let Some(mut config_dir) = dirs::config_dir() {
        config_dir.push(PathBuf::from(&format!("{}/config.toml", meta::NAME)));
        paths.push(config_dir);
    }

    paths.into_iter().find(|path| path.is_file())
}
