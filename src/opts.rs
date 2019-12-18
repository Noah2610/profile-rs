use crate::meta;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(version = meta::VERSION, name = meta::NAME)]
pub struct Opts {
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

impl Opts {
    pub fn new() -> Self {
        Self::from_args()
    }
}
