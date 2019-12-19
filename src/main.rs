#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate serde;
extern crate clap;
extern crate dirs;
extern crate glob;
extern crate hostname;
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
mod profiles;

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
    use files::{edit_files, expand_files, EditSettings, ExpandSettings};

    let context = context::ContextBuilder::default()
        .opts(opts::Opts::new())
        .config(config::Config::new()?)
        .build()?;

    let profiles = context.profiles()?;
    dbg!(&profiles);

    let files = expand_files(
        context.files()?,
        &context.config.files.aliases,
        &ExpandSettings::builder()
            .verbosity(context.verbosity)
            .recurse(context.opts.recurse)
            .build()
            .unwrap(),
    )?;

    edit_files(
        &files,
        EditSettings::builder()
            .keywords(&context.config.keywords)
            .verbosity(context.verbosity)
            .build()
            .unwrap(),
    )?;

    Ok(())
}
