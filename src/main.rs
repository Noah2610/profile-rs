#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate serde;
extern crate clap;
extern crate dirs;
extern crate glob;
extern crate regex;
extern crate serde_regex;
extern crate structopt;
extern crate toml;

mod config;
mod context;
mod error;
mod files;
mod meta;
mod opts;

const ALIAS_PREFIX: &str = "@";

fn main() {
    use std::process::exit;

    match run() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error:\n{}\nExiting.", e);
            exit(1);
        }
    }
}

fn run() -> error::Result<()> {
    use files::{expand_files, ExpandSettings};

    let context = context::ContextBuilder::default()
        .opts(opts::Opts::new())
        .config(config::Config::new()?)
        .build()?;

    let files = expand_files(
        context.files()?,
        &context.config.files.aliases,
        &ExpandSettings::builder()
            .recurse(context.opts.recurse)
            .build()
            .unwrap(),
    )?;

    dbg!(&files);

    Ok(())
}
