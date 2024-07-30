use std::fs::{set_permissions, File, Permissions};
use std::os::unix::fs::PermissionsExt;
use std::process::exit;
use std::{env::args, ffi::CString, path::PathBuf};

use crate::utils::{log, wrap};
use clap::{Args, Parser};
use libc::{S_IRGRP, S_IROTH, S_IRUSR, S_ISVTX, S_IWGRP, S_IWOTH, S_IWUSR, S_IXGRP, S_IXOTH, S_IXUSR};
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

fn get_mode(cli: &Cli) -> u32 {
    let mut mode_bits: u32;
    let input = cli.mode.clone();
    if let Ok(mode) = u32::from_str_radix(&input, 8) {
        mode_bits = mode;
    } else {
        let matches = input.match_indices(['-', '+', '=']);
        if matches.clone().count() != 1 {
            eprintln!("Invalid mode\nSyntax: [ugoa...][[-+=][perms...] or an octal number");
            exit(1);
        };
        let mod_type = match matches.last().unwrap().1 {
            "+" => ModType::Add,
            "-" => ModType::Remove,
            "=" => ModType::ExplicitEquals,
            x => {
                eprintln!("{} is not a valid operator!", x);
                exit(1);
            }
        };
        let parts = input.split_once(['-', '+', '=']).unwrap();
        let mut groups: Vec<ModGroup> = vec![];
        mode_bits = 0;

        for group_char in parts.0.chars() {
            groups.push(match group_char {
                'u' => ModGroup::User,
                'g' => ModGroup::Group,
                'o' => ModGroup::NotInGroup,
                'a' => ModGroup::All,
                _ => {
                    eprintln!("{} is not a valid user/group!", group_char);
                    exit(1);
                }
            })
        }

        for group in groups {
            for perm_char in parts.1.chars() {
                match perm_char {
                    'r' => {
                        mode_bits += match group {
                            ModGroup::User => S_IRUSR,
                            ModGroup::Group => S_IRGRP,
                            ModGroup::NotInGroup => S_IROTH,
                            ModGroup::All => S_IRGRP + S_IROTH + S_IRUSR
                        }
                    }
                    'w' => {
                        mode_bits += match group {
                            ModGroup::User => S_IWUSR,
                            ModGroup::Group => S_IWGRP,
                            ModGroup::NotInGroup => S_IWOTH,
                            ModGroup::All => S_IWGRP + S_IWOTH + S_IWUSR
                        }
                    },
                    'x' => {
                        mode_bits += match group {
                            ModGroup::User => S_IXUSR,
                            ModGroup::Group => S_IXGRP,
                            ModGroup::NotInGroup => S_IXOTH,
                            ModGroup::All => S_IXGRP + S_IXOTH + S_IXUSR
                        }
                    },
                    // Implement later
                    /*'X' => {
                        mode_bits += match group {
                            ModGroup::User => S_IXUSR,
                            ModGroup::Group => S_IXGRP,
                            ModGroup::NotInGroup => S_IXOTH,
                            ModGroup::All => S_IXGRP + S_IXOTH + S_IXUSR
                        }
                    },*/
                    't' => {
                        mode_bits += S_ISVTX
                    },
                    // TODO Implement tomorrow
                    /*
                    'u' => ModPermission::CopyUser,
                    'g' => ModPermission::CopyGroup,
                    'o' => ModPermission::CopyOthers,
                    */
                    _ => {
                        eprintln!("{} is not a valid permission!", perm_char);
                        exit(1);
                    }
                }
            }
        }
    }
    mode_bits
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum ModType {
    Remove,
    Add,
    ExplicitEquals,
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum ModGroup {
    User,
    Group,
    NotInGroup,
    All,
}

fn chmod(cli: &Cli, p: &PathBuf) {
    let mut perms = p.metadata().unwrap().permissions();
    let new_mode = get_mode(cli);
    let destination = wrap(
        if p.is_file() {
            File::options().write(true).open(p)
        } else {
            File::open(p)
        },
        PROGRAM,
    );
    perms.set_mode(new_mode);
    wrap(destination.set_permissions(perms), PROGRAM);
}
