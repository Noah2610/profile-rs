use crate::files::FileList;
use crate::meta;
use crate::profiles::Profile;
use std::convert::TryFrom;
use structopt::StructOpt;

#[derive(Clone, StructOpt, Debug)]
#[structopt(
    name = meta::NAME,
    version = meta::VERSION,
)]
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

    /// For which profile(s) to run.
    /// Defaults to this machine's hostname.
    #[structopt(short, long, parse(from_str = Profile::from))]
    pub profile: Option<Vec<Profile>>,

    /// Files to modify.
    /// If directories are passed, then all of their files are modified (non-recursively).
    /// If the -r option is given, then recursively modify all files in the given directories.
    #[structopt(
        name = "FILES",
        multiple = true,
        parse(try_from_str = FileList::try_from),
    )]
    pub files: Option<FileList>,
}

impl Opts {
    pub fn new() -> Self {
        Self::from_args()
    }
}
