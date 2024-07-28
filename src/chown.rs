use std::{env::args, ffi::CString, path::PathBuf};

use crate::utils::{libc_wrap, log, prompt, wrap, PathExtras, PathType};
use clap::builder::Str;
use clap::{Args, Parser};
use libc::{getgrnam, getpwnam};
use std::os::linux::fs::MetadataExt;
use std::os::unix::fs::chown as unix_chown;
use std::os::unix::fs::lchown as unix_lchown;

const PROGRAM: &str = "chown";

#[derive(Parser, Debug, Clone)]
#[command(
    version,
    about = "Change file (or directory) owner and group\nOWNER and GROUP can either be a name or a numeric GID/UID.\nPass _ (underscore) instead of owner/group to leave it unchanged!\n\nTo use the reference option, just pass _ (underscore) for OWNER:GROUP",
    author = "Alexander Hübner"
)]
struct Cli {
    #[clap(value_parser, value_names(["OWNER:GROUP"]))]
    own_group: String,
    #[clap(value_parser, required = true)]
    files: Vec<PathBuf>,
    // Done
    #[arg(
        short = 'c',
        long = "changes",
        help = "Like verbose but only report when changes are done"
    )]
    changes: bool,
    // Done
    #[arg(
        short = 'f',
        long = "silent",
        alias = "quiet",
        help = "Suppress most error messages"
    )]
    silent: bool,
    // Done
    #[arg(short = 'v', long = "verbose", help = "explain whats being done")]
    verbose: bool,
    // Done
    #[arg(
        long = "dereference",
        help = "Affect the referent of each symbolic link (this is the default), rather than the symbolic link itself",
        conflicts_with("no_dereference")
    )]
    dereference: bool,
    // Done
    #[arg(
        long = "no-dereference",
        help = "Affect the symbolic link instead of the referred file",
        conflicts_with("dereference")
    )]
    no_dereference: bool,
    // Done
    #[arg(
        long = "from",
        value_names(["OWNER:GROUP"]),
        help = "Change the ownership of each file only if its current owner and/or group match those specified here. Either may be omitted, in which case a match is not required for the omitted attribute"
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
    // Done
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
    let perms = get_perms(cli.own_group.clone());
    for file in &cli.files {
        chown(&cli, file, &perms);
    }
}

fn get_id(cli: &Cli, perms: &Perms) -> Id {
    let uid: Option<u32>;
    let gid: Option<u32>;
    let owner = perms.owner.to_owned();
    let group = perms.group.to_owned();
    unsafe {
        if owner == String::from("_") {
            uid = None;
        } else {
            if let Ok(usr_id) = owner.parse::<u32>() {
                uid = Some(usr_id);
            } else {
                let owner = CString::new(owner).unwrap();
                let pw_entry = getpwnam(owner.as_ptr()).read();
                uid = Some(pw_entry.pw_uid);
            }
        };
        if group == String::from("_") {
            gid = None;
        } else {
            if let Ok(grp_id) = group.parse::<u32>() {
                gid = Some(grp_id);
            } else {
                let group = CString::new(group).unwrap();
                let group_entry = getgrnam(group.as_ptr()).read();
                gid = Some(group_entry.gr_gid);
            }
        }
        log(
            cli.verbose && gid.is_none() && uid.is_none() && !cli.changes,
            "Both gid and uid is none, chown has no effect!",
        );
    };
    return Id {
        uid, 
        gid
    }
}

fn get_perms(perm_str: String) -> Perms {
    let raw_perms = perm_str.split_once(":").unwrap_or(("_", "_"));
    return Perms {
        owner: raw_perms.0.to_string(),
        group: raw_perms.1.to_string(),
    };
}

fn chown(cli: &Cli, p: &PathBuf, perms: &Perms) {
    let destination = p.clone();
    let uid: Option<u32>;
    let gid: Option<u32>;
    let from_uid: Option<u32>;
    let from_gid: Option<u32>;
    if let Some(ref_path) = cli.clone().reference {
        let metadata = wrap(ref_path.metadata(), PROGRAM);
        uid = Some(metadata.st_uid());
        gid = Some(metadata.st_gid());
    } else {
        let id = get_id(cli, perms);
        uid = id.uid;
        gid = id.gid;
    };
    if let Some(from_str) = cli.from.clone() {
        let from_perms = get_perms(from_str);
        let id = get_id(cli, &from_perms);
        from_uid = id.uid;
        from_gid = id.gid;
        if from_gid != gid || from_uid != uid {
            return
        }
    }
    log(
        cli.verbose && (gid.is_some() || uid.is_some()),
        format!("Changing ownership of {}", p.display()),
    );
    if cli.no_dereference {
        wrap(unix_lchown(destination, uid, gid), PROGRAM);
    } else {
        wrap(unix_chown(destination, uid, gid), PROGRAM);
    }
}
