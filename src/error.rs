use std::fmt;

pub mod prelude {
    pub use super::Error;
    pub use super::Result;
}

pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    NoConfig,
    ConfigReadError(String, String),
    ConfigParseError(String, String),
    FileNotFound(String),
    FsReadError(String, String),
    AliasNotFound(String),
    GlobError(String, String),
}

impl Error {
    fn message(&self) -> String {
        match self {
            Error::NoConfig => String::from("No config file found"),
            Error::ConfigReadError(file, msg) => {
                format!("Error reading config file: {}\n{}", file, msg)
            }
            Error::ConfigParseError(file, msg) => {
                format!("Error parsing config file: {}\n{}", file, msg)
            }
            Error::FileNotFound(file) => format!("File not found: {}", file),
            Error::FsReadError(dir, msg) => {
                format!("File system read error: {}\n{}", dir, msg)
            }
            Error::AliasNotFound(alias) => {
                format!("Alias is not defined: {}", alias)
            }
            Error::GlobError(pattern, msg) => {
                format!("Invalid glob pattern: {}\n{}", pattern, msg)
            }
        }
    }
}

impl std::error::Error for Error {
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}
