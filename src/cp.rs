use core::fmt;
use std::{
    env::args,
    fs::{
        self, create_dir, create_dir_all, hard_link, metadata, read_link, remove_dir_all,
        remove_file, File, FileTimes,
    },
    os::unix::fs::symlink,
    path::{Path, PathBuf},
    process::exit,
};

use walkdir::WalkDir;

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

    //Done
    #[arg(short = 'a', long = "archive", help = "Same as -dR --preserve=all")]
    archive: bool,
    //Done
    #[arg(long = "attributes-only", help = "")]
    attributes_only: bool,
    //Done
    #[arg(long = "backup", help = "Make a backup of each file")]
    backup_choice: Option<Choice>,

    //Done
    #[arg(
        short = 'b',
        help = "Like --backup but doesn't take an argument (Default option is \"existing\")"
    )]
    backup: bool,

    //TODO Unsure how this would be implemented!
    /*#[arg(
        long = "copy-contents",
        help = "Copy contents of special files when recursive",
        requires("recursive")
    )]
    copy_contents: bool,*/
    //Done
    #[arg(short = 'd', help = "Same as --no-dereference --preserve=links")]
    no_symb_preserve_links: bool,
    //Done
    #[arg(long = "debug", help = "Debug, also activates verbose")]
    debug: bool,

    //Done
    #[command(flatten)]
    destructive_actions: DestructiveActions,
    // Done
    #[arg(short = 'H', help = "Follow command-line symbolic links in SOURCE")]
    follow_symb: bool,
    // Done
    #[arg(
        short = 'l',
        long = "link",
        help = "Hard link files instead of copying"
    )]
    link: bool,
    //Done
    #[arg(
        short = 'L',
        long = "dereference",
        help = "Always follow symbolic links in SOURCE",
        conflicts_with("no_dereference")
    )]
    dereference: bool,
    //Done
    #[arg(
        short = 'P',
        long = "no-dereference",
        help = "Never follow symbolic links in SOURCE",
        conflicts_with("dereference")
    )]
    no_dereference: bool,
    //Done
    #[arg(short = 'p', help = "Same as --preserve=mode,ownership,timestamps")]
    alias_mode_own_time: bool,
    // Done
    #[arg(long = "preserve", help = "Preserve the specified attributes")]
    preserve: Option<Vec<Attributes>>,
    // Done
    #[arg(long = "no-preserve", help = "Don't preserve the specified attributes")]
    no_preserve: Option<Vec<Attributes>>,
    //Done
    #[arg(long = "parents", help = "Use full source file name under DIRECTORY")]
    parents: bool,
    // Done
    #[arg(
        short = 'R',
        long = "recursive",
        help = "Copy directories recursively",
        short_alias('r')
    )]
    recursive: bool,
    // Done
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
    // Done
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

    if cli.no_symb_preserve_links || cli.archive {
        cli.no_dereference = true;
        let mut preserve_copy = cli.preserve.unwrap_or(vec![Attributes::Mode]);
        preserve_copy.push(Attributes::Links);
        cli.preserve = Some(preserve_copy);
    }

    if cli.archive {
        cli.recursive = true;
        let mut preserve_copy = cli.preserve.unwrap_or(vec![Attributes::Mode]);
        preserve_copy.push(Attributes::All);
        cli.preserve = Some(preserve_copy);
    }

    if cli.alias_mode_own_time {
        let mut preserve_copy = cli.preserve.unwrap_or(vec![Attributes::Mode]);
        preserve_copy.append(&mut vec![
            Attributes::Mode,
            Attributes::Timestamps,
            Attributes::Ownership,
        ]);
        cli.preserve = Some(preserve_copy);
    }

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
            _ = wrap(fs::copy(p_clone, backup_path), PROGRAM, false);
        } else {
            let mut i = 0;
            loop {
                backup_path = format!("{}{}{}", cli.destination.display(), suffix, i);
                if !Path::new(&backup_path).exists() {
                    _ = wrap(fs::copy(p_clone, backup_path), PROGRAM, false);
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
                _ = wrap(fs::copy(p_clone, backup_path), PROGRAM, false);
                log(cli.verbose || cli.debug, "Backup successful");
                break;
            }
            i = i + 1;
        }
    } else if choice == Choice::Simple || choice == Choice::Never {
        _ = wrap(fs::copy(p_clone, backup_path), PROGRAM, false);
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
    // Check for destructive actions, and commit necessary follow-up actions
    destructive_check(cli);

    // If we need to remove destination before-hand
    if cli.remove_destination {
        _ = wrap(
            match cli.destination.is_dir() {
                true => remove_dir_all(&cli.destination),
                _ => remove_file(&cli.destination),
            },
            PROGRAM,
            false
        );
    };

    let source;
    if cli.follow_symb && p.is_symlink() {
        source = wrap(read_link(p), PROGRAM, false);
    } else {
        source = p;
    }
    if !cli.recursive || !source.is_dir() {
        debug(
            cli.debug && (cli.dereference || cli.no_dereference),
            "Either dereference or no-dereference was used, has no effect on normal files!",
        );
        log(
            cli.debug || cli.verbose,
            "Normal file, proceeding with normal cp",
        );
        normal_cp(cli, &source)
    } else {
        log(
            cli.debug || cli.verbose,
            "Directory, proceeding with recursive cp",
        );
        recursive_cp(cli, &source)
    }
}

fn normal_cp(cli: &Cli, p: &PathBuf) {
    let mut destination = cli.destination.clone();
    if destination.is_dir() {
        destination.push(p.file_stem().unwrap());
    };

    if cli.parents {
        _ = wrap(create_dir_all(destination.parent().unwrap()), PROGRAM, false);
    };
    if cli.attributes_only {
        match File::create_new(&destination) {
            Err(_) => log(
                cli.debug || cli.verbose,
                "File does exist, just setting attributes instead!",
            ),
            _ => (),
        };
    } else if cli.link {
        _ = wrap(hard_link(p, &destination), PROGRAM, false);
    } else if cli.symbolic_link {
        _ = wrap(symlink(p, &destination), PROGRAM, false);
    } else {
        _ = wrap(fs::copy(p, &destination), PROGRAM, false);
    }
    preserve(
        cli,
        p,
        &destination,
    );
}

fn recursive_cp(cli: &Cli, p: &PathBuf) {
    // Create new root directory
    let mut destination = cli.destination.clone();
    if destination.is_dir() && destination.exists() {
        destination.push(p.file_stem().unwrap());
        dbg!(&destination.exists());
    };
    _ = wrap(create_dir(&destination), PROGRAM, false);
    if cli.parents {
        _ = wrap(create_dir_all(destination.parent().unwrap()), PROGRAM, false);
    };

    for entry in WalkDir::new(&p).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        let newpath = Path::new(&destination).join(&path.strip_prefix(&p).unwrap());
        if path.is_dir() {
            _ = wrap(create_dir_all(newpath), PROGRAM, false);
        } else {
            if cli.attributes_only {
                match File::create_new(&destination) {
                    Err(_) => log(
                        cli.debug || cli.verbose,
                        "File does exist, just setting attributes instead!",
                    ),
                    _ => (),
                };
            } else if cli.link && !cli.dereference {
                _ = wrap(hard_link(path, newpath), PROGRAM, false);
            } else if cli.symbolic_link && !cli.dereference {
                _ = wrap(symlink(path, newpath), PROGRAM, false);
            // If dereference is active we need to read the symlink and copy directly
            // There will never be a situation where both dereference and no-dereference will be
            // active at the same time since clap makes them conflict with each other
            } else if cli.dereference && path.is_symlink() {
                _ = wrap(fs::copy(wrap(read_link(path), PROGRAM, false), newpath), PROGRAM, false);
            } else {
                _ = wrap(fs::copy(path, newpath), PROGRAM, false);
            }
        }
        preserve(
            cli,
            p,
            &destination
        );
    }
}

fn preserve(cli: &Cli, p: &PathBuf, dest: &PathBuf) {
    // Just return of the option isnt used!
    if cli.preserve.is_none() && cli.no_preserve.is_none() {
        return ();
    };

    let destination = wrap(if dest.is_file() {
        File::options().write(true).open(dest)
    } else {
        File::open(dest)
    }, PROGRAM, false);

    let mut preserve_list: Vec<Attributes> = vec![Attributes::Mode];
    // If preserve is specified, overwrite the default
    if cli.preserve.is_some() {
        preserve_list = cli.clone().preserve.unwrap();
    };
    // If no_preserve is specified, remove the items specified from the list
    if cli.no_preserve.is_some() {
        preserve_list.retain(|val| !cli.clone().no_preserve.unwrap().contains(val))
    }

    preserve_list.sort();
    preserve_list.dedup();

    for attribute in preserve_list {
        let source = metadata(p).unwrap();
        // Mode is always preserved automatically, so no need to implement
        //
        // Links will be implemented some time in the future, and "All" is also implemented
        if attribute == Attributes::Ownership || attribute == Attributes::All {
            _ = wrap(destination.set_permissions(source.permissions()), PROGRAM, false);
        }
        if attribute == Attributes::Timestamps || attribute == Attributes::All {
            let mut original_times = FileTimes::new();
            original_times = original_times.set_accessed(source.clone().accessed().unwrap());
            original_times = original_times.set_modified(source.clone().modified().unwrap());
            _ = wrap(destination.set_times(original_times), PROGRAM, false);
        }
    }
}
