#[macro_use]
extern crate serde;
extern crate clap;
extern crate dirs;
extern crate regex;
extern crate serde_regex;
extern crate structopt;
extern crate toml;

mod config;
mod error;
mod file;
mod meta;
mod opts;

fn main() {
    use std::process::exit;

    match run() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    }
}

fn run() -> error::Result<()> {
    let config = config::Config::new()?;
    dbg!(&config);

    let opts = opts::Opts::new();
    dbg!(&opts);

    // let files = if !opts.files.is_empty() {
    //     // opts.files
    // } else {
    //     // config.files.default
    // };

    Ok(())
}
