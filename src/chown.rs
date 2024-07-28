use std::{env::args, ffi::CString, path::PathBuf};

use crate::utils::{is_sudo, libc_wrap, log, prompt, wrap, PathExtras, PathType};
use clap::{Args, Parser};
use libc::{getgrnam, getpwnam, getuid};
use std::os::unix::fs::chown as unix_chown;

const PROGRAM: &str = "chown";

#[derive(Parser, Debug, Clone)]
#[command(
    version,
    about = "Change file (or directory) owner and group\nPass _ (underscore) instead of owner/group to leave it unchanged!",
    author = "Alexander Hübner"
)]
struct Cli {
    #[clap(value_parser)]
    owner: String,
    #[clap(value_parser)]
    group: String,
    #[clap(value_parser, required = true)]
    file: PathBuf,
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
        help = "Affect the referent of each symbolic link (this is the default), rather than the symbolic link itself"
    )]
    dereference: bool,
    // TODO
	#[arg(
        long = "no-dereference",
        help = "Affect the symbolic link instead of the referred file"
    )]
    no_dereference: bool,
    // TODO
	#[arg(
        long = "from",
        help = "Change  the  ownership  of each file only if its current owner and/or group match those specified here. Either may be omitted, in which case a match is not required for the omitted attribute"
    )]
    from: Option<String>,
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
    #[arg(short = 'P', help = "Never traverse any symbolic links")]
    recursive_never: bool,
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

    chown(&cli, cli.clone().file);
}

fn chown(cli: &Cli, p: PathBuf) {
    let destination = p.clone();
    let uid: Option<u32>;
    let gid: Option<u32>;
    unsafe {
        if cli.owner == String::from("_") {
            uid = None;
        } else {
            if let Ok(usr_id) = cli.owner.parse::<u32>() {
                uid = Some(usr_id);
            } else {
                let owner = CString::new(cli.clone().owner).unwrap();
                let pw_entry = getpwnam(owner.as_ptr()).read();
                uid = Some(pw_entry.pw_uid);
            }
        };
        if cli.group == String::from("_") {
            gid = None;
        } else {
            if let Ok(grp_id) = cli.group.parse::<u32>() {
                gid = Some(grp_id);
            } else {
                let group = CString::new(cli.clone().group).unwrap();
                let group_entry = getgrnam(group.as_ptr()).read();
                gid = Some(group_entry.gr_gid);
            }
        }
    };
    wrap(unix_chown(destination, uid, gid), PROGRAM);
}
