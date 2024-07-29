use std::fs::set_permissions;
use std::process::exit;
use std::{env::args, ffi::CString, path::PathBuf};

use crate::utils::{log, wrap};
use clap::{Args, Parser};
use std::os::linux::fs::MetadataExt;
use walkdir::WalkDir;

const PROGRAM: &str = "chmod";

#[derive(Parser, Debug, Clone)]
#[command(
    version,
    about = "Change file (or diretory) mode bits",
    author = "Alexander Hübner"
)]
struct Cli {
    #[clap(value_parser, value_names(["MODE"]))]
    mode: String,
    #[clap(value_parser, num_args = 1.., value_delimiter = ' ', required = true)]
    files: Vec<PathBuf>,
    // TODO
    #[arg(
        short = 'c',
        long = "changes",
        help = "Like verbose but only report when changes are done"
    )]
    changes: bool,
    // TODO
    #[arg(
        short = 'f',
        long = "silent",
        alias = "quiet",
        help = "Suppress most error messages"
    )]
    silent: bool,
    // TODO
    #[arg(short = 'v', long = "verbose", help = "explain whats being done")]
    verbose: bool,
    // TODO
    #[arg(
        long = "dereference",
        help = "Affect the referent of each symbolic link (this is the default), rather than the symbolic link itself",
        conflicts_with("no_dereference")
    )]
    dereference: bool,
    // TODO
    #[arg(
        long = "no-dereference",
        help = "Affect the symbolic link instead of the referred file",
        conflicts_with("dereference")
    )]
    no_dereference: bool,
    // TODO
    #[arg(
        long = "no-preserve-root",
        help = "Dont treat '/' specially (the default)"
    )]
    no_preserve_root: bool,
    // TODO
    #[arg(long = "preserve-root", help = "Fail to operate on '/'")]
    preserve_root: bool,
    // TODO
    #[arg(
        long = "reference",
        help = "Use REFERENCE ownership rather than specifying values, REFERENCE is always dereferenced if a symbolic link"
    )]
    reference: Option<PathBuf>,
    // TODO
    #[arg(short = 'R', long = "recursive", help = "Operate recursively")]
    recursive: bool,
    // TODO
    #[command(flatten)]
    recursive_actions: RecursiveActions,
}

#[derive(Args, Clone, Copy, Debug)]
#[group(required = false, multiple = false)]
struct RecursiveActions {
    // TODO
    #[arg(
        short = 'H',
        help = "If a command line argument is a symbolic link to a directory, traverse it"
    )]
    recursive_dereference: bool,
    // TODO
    #[arg(short = 'L', help = "Traverse every symbolic link found")]
    recursive_traverse: bool,
    // TODO
    #[arg(short = 'P', help = "Never traverse any symbolic links (default)")]
    recursive_never: bool,
}

struct Perms {
    owner: String,
    group: String,
}

struct Id {
    uid: Option<u32>,
    gid: Option<u32>,
}

pub fn main() {
    let mut cli: Cli;
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
    if cli.silent {
        cli.verbose = false;
    }
    for file in &cli.files {
        if cli.preserve_root && file.is_absolute() && file.to_str() == Some("/") {
            eprintln!("Can't operate on / when preserve-root!");
            exit(1);
        };
        chmod(&cli, file);
    }
}

fn chmod(cli: &Cli, p: &PathBuf) {
    // TODO
}
