use std::{env::args, ffi::CString, path::PathBuf};

use crate::utils::check_libc_err;
use clap::{Args, Parser};
use libc::rename;

#[derive(Parser, Debug, Clone)]
#[command(
    version,
    about = "Move (and rename!) files and directories",
    author = "Alexander Hübner"
)]
struct Cli {
    #[clap(value_parser, required = true)]
    source: Vec<PathBuf>,
    #[clap(value_parser, required = true)]
    destination: PathBuf,
    // TODO
    #[arg(long = "backup", help = "Make a backup of each file")]
    backup_choice: Option<Choice>,
    // TODO
    #[arg(
        short = 'b',
        help = "Like --backup but doesnt take an argument (Default option is \"existing\")"
    )]
    backup: bool,
    // TODO
    #[arg(long = "debug", help = "Debug, also activates verbose")]
    debug: bool,
    // TODO
    #[arg(
        long = "exchange",
        help = "Exchange source and destination (swap them)"
    )]
    exchange: bool,
    // TODO
    #[command(flatten)]
    destructive_actions: DestructiveActions,
    // TODO
    #[arg(long = "no-copy", help = "Do not copy if renaming fails")]
    no_copy: bool,
    // TODO
    #[arg(
        long = "skip-trailing-slashes",
        help = "Remove any trailing slashes from each SOURCE argument"
    )]
    skip_trailing_slashes: bool,
    // TODO
    #[arg(
        short = 'S',
        long = "suffix",
        help = "Specify a backup suffix (Text appended to the end of a backup filename)"
    )]
    suffix: Option<String>,
    // TODO
    #[arg(
        short = 't',
        long = "target-directory",
        help = "Move all SOURCE arguments into the specified directory"
    )]
    target_directory: Option<PathBuf>,
    // TODO
    #[arg(
        short = 'T',
        long = "no-target-directory",
        help = "Treat destination as a normal file"
    )]
    no_target_directory: bool,
    // TODO
    #[arg(long = "update", help = "Control which existing files are updated")]
    update: Option<Update>,
    // TODO
    #[arg(short = 'v', long = "verbose", help = "Explain whats being done")]
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
        long = "interactive",
        help = "Prompt before destructive actions, opposite of force"
    )]
    interactive: bool,
    // TODO
    #[arg(
        short = 'n',
        long = "no-clobber",
        help = "Never do any destructive actions"
    )]
    no_clobber: bool,
}

#[derive(clap::ValueEnum, Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Choice {
    /// Never make backups, even if --backup is given
    None,
    /// Alias of none
    Off,
    /// Make numbered backups
    Numbered,
    /// Alias of Numbered
    T,
    /// Make numbered backups if existing, otherwise simple backup
    Existing,
    /// Alias of existing
    Nil,
    /// Always make simple backups
    Simple,
    /// Alias of simple
    Never,
}

#[derive(clap::ValueEnum, Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
enum Update {
    /// Every file in destination is replaced
    #[default]
    All,
    /// No destination files are replaced, wont induce a failure
    None,
    /// Like none, but will induce a failure
    #[clap(name = "none-fail")]
    Nonefail,
    /// Destination files are replaced if they are older than source
    Older,
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
    for p in &cli.source {
        mv(&cli, p)
    }
}

fn mv(cli: &Cli, p: &PathBuf) {
    let source = CString::new(p.to_str().unwrap()).unwrap();
    let dest = CString::new(cli.destination.to_str().unwrap()).unwrap();
    unsafe {
        match check_libc_err(rename(source.as_ptr(), dest.as_ptr())) {
            Ok(_) => (),
            Err(e) => {
                dbg!(e);
            }
        }
    };
}
