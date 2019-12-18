#[macro_use]
extern crate serde;
extern crate clap;
extern crate dirs;
extern crate structopt;
extern crate toml;

mod config;
mod error;
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

    let opt = opts::Opts::new();
    dbg!(&opt);

    Ok(())
}
