use core::fmt;
use std::{
    env::args,
    fs::{create_dir, create_dir_all, set_permissions, Permissions},
    os::unix::fs::PermissionsExt,
    path::PathBuf,
    process::exit,
};

use crate::utils::{debug, log, prompt, wrap};
use clap::{Args, Parser};

const PROGRAM: &str = "rm";

#[derive(Parser, Debug, Clone)]
#[command(
    version,
    about = "Remove (or unlink) files/directories",
    author = "Alexander Hübner"
)]
struct Cli {
    #[clap(value_parser, num_args = 1.., value_delimiter = ' ', required = true)]
    files: Vec<PathBuf>,
    // TODO
    #[command(flatten)]
    destructive_actions: DestructiveActions,
    // TODO
    #[arg(
        long = "one-file-system",
        help = "When removing a hierarchy recursively, skip any directory that is on a file system different from that of the corresponding command line argument"
    )]
    one_file_system: bool,

    #[arg(long = "no-preserve-root", help = "Do not treat '/' specially")]
    no_preserve_root: bool,
    // TODO
    #[arg(
        short = 'R',
        long = "recursive",
        help = "Remove directories recursively",
        short_alias('r')
    )]
    recursive: bool,
    // TODO
    #[arg(
        short = 'd',
        long = "dir",
        help = "Remove empty directories",
    )]
    rm_empty_dir: bool,
    // TODO
    #[arg(
        short = 'v',
        long = "verbose",
        help = "print a message for each created directory"
    )]
    verbose: bool,
}

#[derive(Args, Clone, Copy, Debug)]
#[group(required = false, multiple = false)]
struct DestructiveActions {
    // TODO
    #[arg(
        short = 'f',
        long = "force",
        help = "Do not prompt before destructive actions"
    )]
    force: bool,
    // TODO
    #[arg(
        short = 'i',
        help = "Prompt before destructive actions, opposite of force"
    )]
    interactive: bool,
    // TODO
    #[arg(
        short = 'n',
        long = "no-clobber",
        help = "Never do any destructive actions (silently)"
    )]
    no_clobber: bool,
    // TODO
    #[arg(
        short = 'I',
        help = "Prompt once before removing 3 or more files, or if recursive"
    )]
    interactive_recursive: bool,
    // TODO
    #[arg(
        long = "interactive", 
        help = "Prompt according to the WHEN variable - If no WHEN is specified then always prompt", 
        default_missing_value("always"), 
        num_args=0..=1
    )]
    interactive_when: Option<When>,
}

#[derive(clap::ValueEnum, Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum When {
    /// Never prompt about destructive actions
    Never,
    /// Prompt once, like the -I option
    Once,
    /// Always prompt, like the -i option
    Always,
}

impl fmt::Display for When {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            When::Once => write!(f, "When::Once"),
            When::Always => write!(f, "When::Always"),
            When::Never => write!(f, "When::Never"),
        }
    }
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
