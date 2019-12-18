use crate::file_list::FileList;
use crate::meta;
use std::convert::TryFrom;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(version = meta::VERSION, name = meta::NAME)]
pub struct Opts {
    /// Verbosity. Use multiple options for increased verbosity,
    /// example: -v, -vv, -vvv, etc.
    /// Default verbosity: 0
    /// Max verbosity: TODO!
    #[structopt(short, long, parse(from_occurrences))]
    pub verbose: u8,

    #[structopt(short, long)]
    pub no_modify: bool,

    /// If this flag is set and directories are given as files,
    /// then modify all files in those directories, recursively.
    #[structopt(short, long)]
    pub recurse: bool,

    /// Files to modify.
    /// If directories are passed, then all of their files are modified (non-recursively).
    /// If the -r option is given, then recursively modify all files in the given directories.
    #[structopt(name = "FILES", parse(try_from_str = FileList::try_from))]
    pub files: Vec<FileList>,
}

impl Opts {
    pub fn new() -> Self {
        Self::from_args()
    }
}
