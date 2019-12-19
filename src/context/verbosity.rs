use crate::error::prelude::*;
use std::convert::TryFrom;

#[derive(Clone)]
pub enum Verbosity {
    Quiet = 0,
    Info  = 1,
    Debug = 2,
}

impl Verbosity {
    pub const MIN: u8 = 0;
    pub const MAX: u8 = 2;

    pub fn print_at<S, V>(&self, min_verbosity: V, msg: S)
    where
        S: std::fmt::Display,
        V: Into<Verbosity>,
    {
        let current = self.value();
        let min = min_verbosity.into().value();
        if current >= min {
            println!("{}", msg);
        }
    }

    fn value(&self) -> u8 {
        match self {
            Verbosity::Quiet => 0,
            Verbosity::Info => 1,
            Verbosity::Debug => 2,
        }
    }
}

impl Default for Verbosity {
    fn default() -> Self {
        Verbosity::Quiet
    }
}

impl TryFrom<u8> for Verbosity {
    type Error = Error;

    fn try_from(n: u8) -> Result<Self> {
        match n {
            0 => Ok(Verbosity::Quiet),
            1 => Ok(Verbosity::Info),
            2 => Ok(Verbosity::Debug),
            _ => Err(Error::InvalidVerbosity(n)),
        }
    }
}
