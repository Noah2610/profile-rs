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
mod error;
mod file_list;
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
    use file_list::{expand_files, ExpandSettings};

    let opts = opts::Opts::new();
    dbg!(&opts);

    let config = config::Config::new()?;

    let files = if !opts.files.is_empty() {
        opts.files.into()
    } else if !config.files.default.is_empty() {
        config.files.default.into()
    } else {
        // TODO
        panic!("TODO");
    };

    let files = expand_files(
        &files,
        &config.files.aliases,
        &ExpandSettings::builder()
            .recurse(opts.recurse)
            .build()
            .unwrap(),
    )?;

    dbg!(&files);

    Ok(())
}
