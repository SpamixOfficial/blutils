use core::fmt;
use std::{
    env::args,
    ffi::CString,
    fs::{self, hard_link, read_link, remove_dir_all, remove_file},
    os::unix::fs::symlink,
    path::{Path, PathBuf},
    process::exit,
};

//use pathdiff::diff_paths;

use crate::utils::{is_sudo, libc_wrap, log, prompt, wrap, PathExtras, PathType};
use clap::{Args, Parser};
use libc::{linkat, AT_FDCWD};

const PROGRAM: &str = "ln";

#[derive(Parser, Debug, Clone)]
#[command(
    version,
    about = "Link files and directories",
    author = "Alexander Hübner"
)]
struct Cli {
    #[clap(value_parser, required = true)]
    source: Vec<PathBuf>,
    #[clap(value_parser, required = true)]
    destination: PathBuf,

    // Done
    #[arg(long = "backup", help = "Make a backup of each file")]
    backup_choice: Option<Choice>,

    // Done
    #[arg(
        short = 'b',
        help = "Like --backup but doesnt take an argument (Default option is \"existing\")"
    )]
    backup: bool,
    // Done
    #[arg(
        short = 'd',
        long = "directory",
        short_alias('F'),
        help = "Allow the superuser to attempt to hard link directories (this will probably fail due to system restrictions, even for the superuser)"
    )]
    try_hard_link_dir_sudo: bool,
    // Done
    #[command(flatten)]
    destructive_actions: DestructiveActions,
    // Done
    #[arg(
        short = 'L',
        long = "logical",
        help = "Dereference SOURCEs that are symbolic links"
    )]
    logical: bool,
    // Done
    #[arg(
        short = 'n',
        long = "no-dereference",
        help = "Never follow symbolic links in SOURCE",
        conflicts_with("logical")
    )]
    no_dereference: bool,
    // Done
    #[arg(
        short = 'P',
        long = "physical",
        help = "Make hard links directly to symbolic links",
        conflicts_with("symbolic_link")
    )]
    physical: bool,
    // TODO
    /*#[arg(
        short = 'r',
        long = "relative",
        help = "With -s, create links relative to link location",
        requires("symbolic_link")
    )]
    relative: bool,*/
    // Done
    #[arg(
        short = 's',
        long = "symbolic-link",
        help = "Make symbolic links instead of hard linking"
    )]
    symbolic_link: bool,
    // Done
    #[arg(
        short = 'S',
        long = "suffix",
        help = "Specify a backup suffix (Text appended to the end of a backup filename)"
    )]
    suffix: Option<String>,

    // Done
    #[arg(
        short = 't',
        long = "target-directory",
        help = "Treat destination as a directory",
        conflicts_with("no_target_directory")
    )]
    target_directory: bool,

    // Done
    // Default behaviour so no need to implement really!
    #[arg(
        short = 'T',
        long = "no-target-directory",
        help = "Treat destination as a normal file",
        conflicts_with("target_directory")
    )]
    no_target_directory: bool,
    // Done
    #[arg(short = 'v', long = "verbose", help = "explain whats being done")]
    verbose: bool,
}

#[derive(Args, Clone, Copy, Debug)]
#[group(required = false, multiple = false)]
struct DestructiveActions {
    // Done
    #[arg(
        short = 'f',
        long = "force",
        help = "Do not prompt before destructive actions"
    )]
    force: bool,
    // Done
    #[arg(
        short = 'i',
        long = "interactive",
        help = "Prompt before destructive actions, opposite of force"
    )]
    interactive: bool,
    // Done
    #[arg(
        short = 'N',
        long = "no-clobber",
        help = "Never do any destructive actions (silently)"
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

impl fmt::Display for Choice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Choice::Existing | Choice::Nil => write!(f, "Choice::Existing/Choice::Nil"),
            Choice::None | Choice::Off => write!(f, "Choice::None/Choice::Off"),
            Choice::Numbered | Choice::T => write!(f, "Choice::Numbered/Choice::T"),
            Choice::Simple | Choice::Never => write!(f, "Choice::Simple/Choice::Never"),
        }
    }
}

pub fn main() {
    let cli: Cli;
    // skip first arg if it happens to be "blutils"
    is_sudo();
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

    for mut p in cli.source.clone() {
        log(cli.verbose, format!("Linking {}", p.display()));
        p = backup(&cli, p);
        ln(&cli, p);
    }
}

fn backup(cli: &Cli, p: PathBuf) -> PathBuf {
    // Checking for options and if the file exists
    let destination = cli.destination.clone();
    if (!cli.backup && !cli.backup_choice.is_some()) || destination.try_exists().is_err() {
        return p;
    };
    let p_clone = p.clone();
    let suffix = cli.suffix.clone().unwrap_or(String::from("~"));
    let mut backup_path = format!("{}{}", destination.display(), suffix);
    let choice = cli.backup_choice.unwrap_or(Choice::Existing);

    log(
        cli.verbose,
        format!("Starting backup with choice {}", choice),
    );

    if choice == Choice::Nil || choice == Choice::Existing {
        if !Path::new(&backup_path).exists() {
            _ = wrap(fs::copy(p_clone, backup_path), PROGRAM);
        } else {
            let mut i = 0;
            loop {
                backup_path = format!("{}{}{}", destination.display(), suffix, i);
                if !Path::new(&backup_path).exists() {
                    _ = wrap(fs::copy(p_clone, backup_path), PROGRAM);
                    log(cli.verbose, "Backup successful");
                    break;
                }
                i = i + 1;
            }
        }
    } else if choice == Choice::Numbered || choice == Choice::T {
        let mut i = 0;
        loop {
            backup_path = format!("{}{}{}", destination.display(), suffix, i);
            if !Path::new(&backup_path).exists() {
                _ = wrap(fs::copy(p_clone, backup_path), PROGRAM);
                log(cli.verbose, "Backup successful");
                break;
            }
            i = i + 1;
        }
    } else if choice == Choice::Simple || choice == Choice::Never {
        _ = wrap(fs::copy(p_clone, backup_path), PROGRAM);
        log(cli.verbose, "Backup successful");
    }
    return p;
}

fn destructive_check(cli: &Cli) {
    let destination = cli.destination.clone();
    if cli.destructive_actions.force {
        return;
    }
    if cli.destructive_actions.no_clobber && destination.exists() {
        eprintln!("ln: Error: About to commit destructive action - not allowed, exiting!");
        exit(1);
    } else if destination.exists() && cli.destructive_actions.interactive {
        if !prompt(
            format!(
                "Destructive action: {} exists and will be overwritten. Continue? ",
                destination.display()
            ),
            false,
        ) {
            exit(0)
        }
    }
}

fn ln(cli: &Cli, path: PathBuf) {
    destructive_check(cli);

    let mut destination = cli.destination.clone();
    let mut p = path.clone();
    if destination.is_dir() || (cli.target_directory && !cli.no_target_directory) {
        destination = destination.join(p.clone());
    };

    if p.is_symlink() && cli.logical {
        p = wrap(read_link(p), PROGRAM);
    }

    if cli.destructive_actions.force {
        log(cli.verbose, "Force was used, removing destination!");
        wrap(
            match p.as_path().ptype() {
                PathType::File | PathType::Symlink => remove_file(&destination),
                _ => remove_dir_all(&destination),
            },
            PROGRAM,
        )
    }

    if cli.symbolic_link {
        /*if cli.relative {
            dbg!(&destination, &p);
            dbg!(diff_paths(&p, &destination).unwrap());
        };*/
        slink(p, destination);
    } else {
        link(cli, p, destination);
    }
}

// Function for handling symbolic links
//
// Hard and Symb links are split into 2 different functons because they both have some different
// options which are unique to them.
//
// Yk, keep it clean :-)
fn slink(p: PathBuf, destination: PathBuf) {
    wrap(symlink(p, destination), PROGRAM);
}

fn link(cli: &Cli, p: PathBuf, destination: PathBuf) {
    if p.is_dir() && !cli.try_hard_link_dir_sudo && !is_sudo() {
        eprintln!("ln: Error: Can't hard link directories!");
        exit(1);
    };

    if cli.physical && !cli.logical {
        let source = CString::new(p.to_str().to_owned().unwrap().to_string()).unwrap();
        let dest = CString::new(destination.to_str().to_owned().unwrap().to_string()).unwrap();
        unsafe {
            wrap(
                libc_wrap(linkat(
                    AT_FDCWD,
                    source.as_ptr(),
                    AT_FDCWD,
                    dest.as_ptr(),
                    0,
                )),
                PROGRAM,
            )
        };
    } else {
        wrap(hard_link(p, destination), PROGRAM)
    };
}
