extern crate clap;
#[macro_use]
extern crate structopt;

use std::path::PathBuf;
use structopt::StructOpt;

mod meta {
    pub const NAME: &str = env!("CARGO_PKG_NAME");
    pub const VERSION: &str = env!("CARGO_PKG_VERSION");
}

#[derive(StructOpt, Debug)]
#[structopt(version = meta::VERSION, name = meta::NAME)]
struct Opts {
    /// Verbosity. Use multiple options for increased verbosity,
    /// example: -v, -vv, -vvv, etc.
    /// Default verbosity: 0
    /// Max vecbosity: TODO!
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,

    #[structopt(short, long)]
    no_modify: bool,

    /// If this flag is set and directories are given as files,
    /// then modify all files in those directories, recursively.
    #[structopt(short, long)]
    recurse: bool,

    /// Files to modify.
    /// If directories are passed, then all of their files are modified (non-recursively).
    /// If the -r option is given, then recursively modify all files in the given directories.
    #[structopt(name = "FILES", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn main() {
    let opt = Opts::from_args();
    dbg!(&opt);
}
