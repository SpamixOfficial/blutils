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

    //TODO
    #[arg(short = 'a', long = "archive", help = "Same as -dR --preserve=all")]
    archive: bool,
    //TODO
    #[arg(long = "attributes-only", help = "")]
    attributes_only: bool,
    //Done
    #[arg(long = "backup", help = "Make a backup of each file")]
    backup_choice: Option<Choice>,

    //Done
    #[arg(
        short = 'b',
        help = "Like --backup but doesnt take an argument (Default option is \"existing\")"
    )]
    backup: bool,

    //TODO
    #[arg(
        long = "copy-contents",
        help = "Copy contents of special files when recursive"
    )]
    copy_contents: bool,
    //TODO
    #[arg(short = 'd', help = "Same as --no-dereference --preserve=links")]
    no_symb_preserve_links: bool,
    //Done
    #[arg(long = "debug", help = "Debug, also activates verbose")]
    debug: bool,

    //TODO
    #[command(flatten)]
    destructive_actions: DestructiveActions,
    //TODO
    #[arg(short = 'H', help = "Follow command-line symbolic links in SOURCE")]
    follow_symb: bool,
    //TODO
    #[arg(
        short = 'l',
        long = "link",
        help = "Hard link files instead of copying"
    )]
    link: bool,
    //TODO
    #[arg(
        short = 'L',
        long = "dereference",
        help = "Always follow symbolic links in SOURCE",
        conflicts_with("no_dereference")
    )]
    dereference: bool,
    //TODO
    #[arg(
        short = 'P',
        long = "no-dereference",
        help = "Never follow symbolic links in SOURCE",
        conflicts_with("dereference")
    )]
    no_dereference: bool,
    //TODO
    #[arg(short = 'p', help = "Same as --preserve=mode,ownership,timestamps")]
    alias_mode_own_time: bool,
    //TODO
    #[arg(long = "preserve", help = "Preserve the specified attributes")]
    preserve: Option<Vec<Attributes>>,
    //TODO
    #[arg(long = "no-preserve", help = "Don't preserve the specified attributes")]
    no_preserve: Option<Vec<Attributes>>,
    //TODO
    #[arg(long = "parents", help = "Use full source file name under DIRECTORY")]
    parents: bool,
    //TODO
    #[arg(
        short = 'R',
        long = "recursive",
        help = "Copy directories recursively",
        short_alias('r')
    )]
    recursive: bool,
    //TODO
    #[arg(
        long = "remove-destination",
        help = "Remove each existing destination file before attempting to open it (contrast with --force)"
    )]
    remove_destination: bool,
    // Done
    #[arg(
        long = "strip-trailing-slashes",
        help = "Remove any trailing slashes from each SOURCE argument"
    )]
    strip_trailing_slashes: bool,
    //TODO
    #[arg(
        short = 's',
        long = "symbolic-link",
        help = "Make symbolic links instead of copying"
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
        help = "Treat destination as a directory"
    )]
    target_directory: bool,

    // Done
    #[arg(
        short = 'T',
        long = "no-target-directory",
        help = "Treat destination as a normal file"
    )]
    no_target_directory: bool,
    // Planned for later updates
    //#[arg(long = "update", help = "Control which existing files are updated")]
    //update: Option<Update>,
    #[arg(short = 'v', long = "verbose", help = "explain whats being done")]
    verbose: bool,
    //TODO
    #[arg(
        long = "keep-directory-symlink",
        help = "Follow existing symlinks to directories"
    )]
    keep_symlinks: bool,
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
        help = "Never do any destructive actions (silently)"
    )]
    no_clobber: bool,
}

#[derive(clap::ValueEnum, Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Attributes {
    /// Preserve permissions
    Mode,
    /// Preserve user and groups
    Ownership,
    /// Preserve all timestamps
    Timestamps,
    /// Preserve hard links
    Links,
    /// Preserve everything
    All,
}

impl fmt::Display for Attributes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Attributes::Mode => write!(f, "Attributes::Mode"),
            Attributes::All => write!(f, "Attributes::All"),
            Attributes::Links => write!(f, "Attributes::Links"),
            Attributes::Timestamps => write!(f, "Attributes::Timestamps"),
            Attributes::Ownership => write!(f, "Attributes::Ownership"),
        }
    }
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
    for mut p in cli.source.clone() {
        log(cli.verbose || cli.debug, format!("Moving {}", p.display()));
        p = slashes(&cli, p);
        p = backup(&cli, p);
        cp(&cli, p);
    }
}

fn backup(cli: &Cli, p: PathBuf) -> PathBuf {
    // Checking for options and if the file exists
    if (!cli.backup && !cli.backup_choice.is_some()) || cli.destination.try_exists().is_err() {
        return p;
    };
    let p_clone = p.clone();
    let suffix = cli.suffix.clone().unwrap_or(String::from("~"));
    let mut backup_path = format!("{}{}", cli.destination.display(), suffix);
    let choice = cli.backup_choice.unwrap_or(Choice::Existing);

    log(
        cli.verbose || cli.debug,
        format!("Starting backup with choice {}", choice),
    );

    if choice == Choice::Nil || choice == Choice::Existing {
        if !Path::new(&backup_path).exists() {
            _ = wrap(fs::copy(p_clone, backup_path), PROGRAM);
        } else {
            let mut i = 0;
            loop {
                backup_path = format!("{}{}{}", cli.destination.display(), suffix, i);
                if !Path::new(&backup_path).exists() {
                    _ = wrap(fs::copy(p_clone, backup_path), PROGRAM);
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
                _ = wrap(fs::copy(p_clone, backup_path), PROGRAM);
                log(cli.verbose || cli.debug, "Backup successful");
                break;
            }
            i = i + 1;
        }
    } else if choice == Choice::Simple || choice == Choice::Never {
        _ = wrap(fs::copy(p_clone, backup_path), PROGRAM);
        log(cli.verbose || cli.debug, "Backup successful");
    }
    return p;
}

fn destructive_check(cli: &Cli) {
    if cli.destructive_actions.no_clobber && cli.destination.exists() {
        eprintln!("mv: Error: About to commit destructive action - not allowed, exiting!");
        exit(1);
    } else if cli.destination.exists() && cli.destructive_actions.interactive {
        if !prompt(
            format!(
                "Destructive action: {} exists and will be overwritten. Continue? ",
                cli.destination.display()
            ),
            false,
        ) {
            exit(0)
        }
    }
}

fn slashes(cli: &Cli, p: PathBuf) -> PathBuf {
    let source;
    if cli.strip_trailing_slashes || cli.no_target_directory {
        // Copy into a string since we need string manipulation for this!
        let mut source_copy = p.to_str().to_owned().unwrap().to_string();
        while source_copy.ends_with("/") {
            // Discard the result, we dont really care about it ¯\_(ツ)_/¯
            _ = source_copy.pop()
        }
        // When it doesnt end with a slash the loop ends and we create a CString from our new
        // string
        source = PathBuf::from(source_copy);
    } else if cli.target_directory {
        let mut source_copy = p.to_str().to_owned().unwrap().to_string();
        if !source_copy.ends_with("/") {
            source_copy.push('/');
        };
        source = PathBuf::from(source_copy);
    } else {
        return p;
    };
    return source;
}

fn cp(cli: &Cli, p: PathBuf) {
    destructive_check(cli);
    _ = wrap(fs::copy(p, &cli.destination), PROGRAM);
}
