use crate::context::Verbosity;
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
    NoFiles,
    InvalidVerbosity(u8),

    UnkownError(String),
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
            Error::NoFiles => format!(
                "No files given. Pass files as arguments or set the \
                 [files.default] value in the config"
            ),
            Error::InvalidVerbosity(n) => format!(
                "Invalid verbosity level: {}\nVerbosity can only go from {} \
                 to {}",
                n,
                Verbosity::MIN,
                Verbosity::MAX,
            ),

            Error::UnkownError(msg) => msg.to_string(),
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

impl<S: Into<String>> From<S> for Error {
    fn from(s: S) -> Self {
        Error::UnkownError(s.into())
    }
}
