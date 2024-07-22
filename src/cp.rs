use core::fmt;
use std::{
    env::args,
    ffi::CString,
    fs::{self, remove_file},
    path::{Path, PathBuf},
    process::exit,
};

use crate::utils::{debug, log, prompt, wrap};
use clap::{Args, Parser};

const PROGRAM: &str = "cp";

#[derive(Parser, Debug, Clone)]
#[command(
    version,
    about = "Copy SOURCE to (or into) DESTINATION",
    author = "Alexander Hübner"
)]
struct Cli {
    #[clap(value_parser, required = true)]
    source: Vec<PathBuf>,
    #[clap(value_parser, required = true)]
    destination: PathBuf,

    #[arg(short = 'a', long = "archive", help = "Same as -dR --preserve=all")]
    #[arg(long = "attributes-only", help = "")]

    #[arg(long = "backup", help = "Make a backup of each file")]
    backup_choice: Option<Choice>,

    #[arg(
        short = 'b',
        help = "Like --backup but doesnt take an argument (Default option is \"existing\")"
    )]
    backup: bool,
    
    #[arg(short = '', long = "", help = "")]
    #[arg(short = '', long = "", help = "")]
    #[arg(long = "debug", help = "Debug, also activates verbose")]
    debug: bool,

    #[command(flatten)]
    destructive_actions: DestructiveActions,
    #[arg(short = '', long = "", help = "")]

    #[arg(long = "no-copy", help = "Do not copy if renaming fails")]
    no_copy: bool,

    #[arg(
        long = "strip-trailing-slashes",
        help = "Remove any trailing slashes from each SOURCE argument"
    )]
    strip_trailing_slashes: bool,

    #[arg(
        short = 'S',
        long = "suffix",
        help = "Specify a backup suffix (Text appended to the end of a backup filename)"
    )]
    suffix: Option<String>,

    #[arg(
        short = 't',
        long = "target-directory",
        help = "Treat destination as a directory"
    )]
    target_directory: bool,

    #[arg(
        short = 'T',
        long = "no-target-directory",
        help = "Treat destination as a normal file"
    )]
    no_target_directory: bool,
    // Planned for later updates
    //#[arg(long = "update", help = "Control which existing files are updated")]
    //update: Option<Update>,
    #[arg(short = 'v', long = "verbose", help = "Explain whats being done")]
    verbose: bool,
}

#[derive(Args, Clone, Copy, Debug)]
#[group(required = false, multiple = false)]
struct DestructiveActions {
    #[arg(
        short = 'f',
        long = "force",
        help = "Do not prompt before destructive actions"
    )]
    force: bool,
    #[arg(
        short = 'i',
        long = "interactive",
        help = "Prompt before destructive actions, opposite of force"
    )]
    interactive: bool,
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

impl fmt::Display for Choice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Choice::Existing | Choice::Nil => write!(f, "Existing/Nil"),
            Choice::None | Choice::Off => write!(f, "None/Off"),
            Choice::Numbered | Choice::T => write!(f, "Numbered/T"),
            Choice::Simple | Choice::Never => write!(f, "Simple/Never"),
        }
    }
}
// Planned for later releases
/*#[derive(clap::ValueEnum, Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
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
}*/

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
        log(cli.verbose || cli.debug, format!("Moving {}", p.display()));
        backup(&cli, p);
        cp(&cli, p);
    }
}

fn backup(cli: &Cli, p: &PathBuf) {
    // Checking for options and if the file exists
    if (!cli.backup && !cli.backup_choice.is_some()) || cli.destination.try_exists().is_err() {
        return;
    };

    let suffix = cli.suffix.clone().unwrap_or(String::from("~"));
    let mut backup_path = format!("{}{}", cli.destination.display(), suffix);
    let choice = cli.backup_choice.unwrap_or(Choice::Existing);

    log(
        cli.verbose || cli.debug,
        format!("Starting backup with choice {}", choice),
    );

    if choice == Choice::Nil || choice == Choice::Existing {
        if !Path::new(&backup_path).exists() {
            _ = wrap(fs::copy(p, backup_path), PROGRAM);
        } else {
            let mut i = 0;
            loop {
                backup_path = format!("{}{}{}", cli.destination.display(), suffix, i);
                if !Path::new(&backup_path).exists() {
                    _ = wrap(fs::copy(p, backup_path), PROGRAM);
                    log(cli.verbose || cli.debug, "Backup successful");
                    break;
                }
                i = i + 1;
            }
        }
    } else if choice == Choice::Numbered || choice == Choice::T {
        let mut i = 0;
        loop {
            backup_path = format!("{}{}{}", cli.destination.display(), suffix, i);
            if !Path::new(&backup_path).exists() {
                _ = wrap(fs::copy(p, backup_path), PROGRAM);
                log(cli.verbose || cli.debug, "Backup successful");
                break;
            }
            i = i + 1;
        }
    } else if choice == Choice::Simple || choice == Choice::Never {
        _ = wrap(fs::copy(p, backup_path), PROGRAM);
        log(cli.verbose || cli.debug, "Backup successful");
    }
}

fn cp(cli: &Cli, p: &PathBuf) {}
