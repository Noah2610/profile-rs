mod verbosity;

pub use verbosity::Verbosity;

use crate::config::Config;
use crate::error::prelude::*;
use crate::files::FileList;
use crate::opts::Opts;
use std::convert::TryFrom;

#[derive(Builder)]
#[builder(pattern = "owned", setter(into), build_fn(skip))]
pub struct Context {
    pub opts:   Opts,
    pub config: Config,

    #[builder(setter(skip))]
    pub verbosity: Verbosity,
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

impl ContextBuilder {
    pub fn build(self) -> Result<Context> {
        let opts = self.opts.ok_or("opts must be initialized")?;
        let verbosity = Verbosity::try_from(opts.verbose)?;

        Ok(Context {
            opts,
            config: self.config.ok_or("config must be initialized")?,

            verbosity,
        })
    }
}
