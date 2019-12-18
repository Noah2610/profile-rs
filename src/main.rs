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
    let config = config::Config::new()?;
    dbg!(&config);

    let opts = opts::Opts::new();
    dbg!(&opts);

    let files =
        file_list::expand_files(&opts.files.into(), &config.files.aliases);

    dbg!(&files);

    Ok(())
}
