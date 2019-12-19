use crate::config::Config;
use crate::error::prelude::*;
use crate::files::FileList;
use crate::opts::Opts;

#[derive(Builder)]
#[builder(setter(into))]
pub struct Context {
    pub opts:   Opts,
    pub config: Config,
}

impl Context {
    pub fn files(&self) -> Result<&FileList> {
        self.opts
            .files
            .as_ref()
            .or_else(|| {
                if !self.config.files.default.is_empty() {
                    Some(&self.config.files.default)
                } else {
                    None
                }
            })
            .ok_or(Error::NoFiles)
    }
}
