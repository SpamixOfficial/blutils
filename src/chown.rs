use std::env::args;

use crate::utils::{is_sudo, libc_wrap, log, prompt, wrap, PathExtras, PathType};
use clap::{Args, Parser};
use libc::{linkat, AT_FDCWD};

const PROGRAM: &str = "chown";

#[derive(Parser, Debug, Clone)]
#[command(
    version,
    about = "Change file (or directory) owner and group",
    author = "Alexander Hübner"
)]
struct Cli {
    #[clap(value_parser, required_if_eq("reference", "None"))]
    owner: String,
    #[clap(value_parser, required = true)]
    file: PathBuf,
    #[arg(short = 'c', long = "changes", help = "Like verbose but only report when changes are done")]
    changes: bool,
    #[arg(short = 'f', long = "silent", long_alias = "quiet", help = "Suppress most error messages")]
    silent: bool,
    #[arg(short = 'v', long = "verbose", help = "explain whats being done")]
    verbose: bool,
    #[arg(long = "dereference", help = "Affect the referent of each symbolic link (this is the default), rather than the symbolic link itself")]
    dereference: bool,
    #[arg(short = 'h', long = "no-dereference", help = "Affect the symbolic link instead of the referred file")]
    no_dereference: bool,
    #[arg(long = "from", help = "Change  the  ownership  of each file only if its current owner and/or group match those specified here. Either may be omitted, in which case a match is not required for the omitted attribute")]
    from: Option<String>,
    #[arg(long = "no-preserve-root", help = "Dont treat '/' specially (the default)")]
    no_preserve_root: bool,
    #[arg(long = "preserve-root", help = "Fail to operate on '/'")]
    preserve_root: bool,
    #[arg(long = "reference", help = "Use REFERENCE ownership rather than specifying values, REFERENCE is always dereferenced if a symbolic link")]
    reference: Option<PathBuf>,
    #[arg(short = 'R', long = "recursive", help = "Operate recursively")]
    recursive: bool,
    #[command(flatten)]
    recursive_actions: RecursiveActions,
}

#[derive(Args, Clone, Copy, Debug)]
#[group(required = false, multiple = false)]
struct RecursiveActions {
    // Done
    #[arg(
        short = 'H',
        help = "If a command line argument is a symbolic link to a directory, traverse it"
    )]
    recursive_dereference: bool,
    // Done
    #[arg(
        short = 'L',
        help = "Traverse every symbolic link found"
    )]
    recursive_traverse: bool,
    // Done
    #[arg(
        short = 'P',
        help = "Never traverse any symbolic links"
    )]
    recursive_never: bool,
}


pub fn main() {
    let cli: Cli;
    // skip first arg if it happens to be "blutils"
    if args().collect::<Vec<String>>()[0]
        .split("/")
        .last()
        .unwrap()
        == "blutils"
    {
        cli = Cli::parse_from(args().skip(1));
    } else {
        cli = Cli::parse();
    }; 
}


