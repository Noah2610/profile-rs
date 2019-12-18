use crate::error::prelude::*;
use std::convert::TryFrom;
use std::path::PathBuf;

const ALIAS_PREFIX: &str = "@";

#[derive(Debug)]
pub enum File {
    File(PathBuf),
    Alias(String),
}

impl TryFrom<&str> for File {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self> {
        if s.starts_with(ALIAS_PREFIX) {
            Ok(File::Alias(s.to_string()))
        } else {
            let path = PathBuf::from(s);
            if path.exists() {
                Ok(File::File(path))
            } else {
                Err(Error::FileNotFound(s.to_string()))
            }
        }
    }
}
