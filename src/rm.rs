use core::fmt;
use std::{
    env::args,
    fs::{remove_dir, remove_file},
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    process::exit,
};

use crate::utils::{log, prompt, wrap, PathExtras, PathType};
use clap::{Args, Parser};
use walkdir::WalkDir;

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
    // Done
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
    // Done
    #[arg(
        short = 'R',
        long = "recursive",
        help = "Remove directories recursively",
        short_alias('r')
    )]
    recursive: bool,
    // Done
    #[arg(
        short = 'd',
        long = "dir",
        help = "Remove empty directories",
        requires("recursive")
    )]
    rm_empty_dir: bool,
    // Done
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
        help = "Prompt before destructive actions, opposite of force"
    )]
    interactive: bool,
    // Done
    #[arg(
        short = 'I',
        help = "Prompt once before removing 3 or more files, or if recursive"
    )]
    interactive_recursive: bool,
    // Done
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
    destructive_handle(&cli, None);
    for p in &cli.files {
        rm(&cli, p);
    }
}

fn destructive_handle(cli: &Cli, path: Option<&PathBuf>) {
    let p: &PathBuf;
    if let Some(x) = path {
        p = x;
    } else {
        if (cli.destructive_actions.interactive_recursive
            || cli
                .destructive_actions
                .interactive_when
                .is_some_and(|x| x == When::Once))
            && (cli.files.len() >= 3 || cli.recursive)
        {
            if !prompt(format!("Destructive action: You are about to remove more than 1 file, are you sure about this?"), false) {
            exit(0);}
        }
        return;
    }
    if cli.destructive_actions.force || cli.destructive_actions.interactive_when.is_some_and(|x| x == When::Never) {
        log(
            cli.verbose,
            "Force enabled, skipping any checks and just proceeding",
        );
        return;
    }

    if !p.exists() {
        log(cli.verbose, "File doesnt exist, returning and fail later");
        return;
    }

    if write_protection(p) {
        if !prompt(
            format!(
                "Destructive action: {} is write protected. Continue with removal? ",
                p.display()
            ),
            false,
        ) {
            exit(0)
        }
    }
    if cli.destructive_actions.interactive
        || cli
            .destructive_actions
            .interactive_when
            .is_some_and(|x| x == When::Always)
    {
        if !prompt(
            format!(
                "Destructive action: {} will be removed. Continue? ",
                p.display()
            ),
            false,
        ) {
            exit(0)
        }
    }
}

fn rm(cli: &Cli, p: &PathBuf) {
    if cli.destructive_actions.force
        && cli.recursive
        && p == Path::new("/")
        && !cli.no_preserve_root
    {
        println!("HEADS UP! You are trying to remove the root directory of your system.\nThis is not possible without no-preserve-root.\n\nYOU ARE NOT REMOVING THE FRENCH LANGUAGE PACK, YOU ARE REMOVING YOUR SYSTEM");
        exit(0);
    };

    // Handle destructive options
    Some(destructive_handle(cli, Some(p)));

    if !cli.recursive {
        normal_rm(cli, p);
    } else {
        recursive_rm(cli, p);
    }
}

fn normal_rm(cli: &Cli, p: &PathBuf) {
    log(
        cli.verbose,
        format!("Removing {} {}...", p.type_display(), p.display()),
    );
    if cli.rm_empty_dir && p.is_dir() {
        if p.read_dir().unwrap().next().is_none() {
                _ = wrap(remove_dir(p), PROGRAM);
                return;
        };
    }
    _ = wrap(remove_file(p), PROGRAM);
}

fn recursive_rm(cli: &Cli, p: &PathBuf) {
    for entry in WalkDir::new(&p)
        .contents_first(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        log(
            cli.verbose,
            format!("Removing {} {}...", path.type_display(), path.display()),
        );
        dbg!(format!(
            "{}",
            &path.metadata().unwrap().permissions().mode() & 0o777
        ));
        /*match path.ptype() {
            PathType::File | PathType::Symlink => wrap(remove_file(path), PROGRAM),
            PathType::Directory => wrap(remove_dir(path), PROGRAM)
        }*/
    }
}

fn write_protection(p: &PathBuf) -> bool {
    let perms = get_mode(p);
    if perms.Owner == 4 && perms.Others == 4 && perms.Group == 4 {
        return true;
    } else {
        return false;
    }
}

fn get_mode(p: &PathBuf) -> FilePerm {
    let raw_mode = format!("{:o}", &p.metadata().unwrap().permissions().mode());
    return {
        let ints: Vec<u32> = raw_mode
            .split_at(raw_mode.len() - 3)
            .1
            .split("")
            .filter_map(|f| f.parse::<u32>().ok())
            .collect();
        FilePerm {
            Owner: ints.get(0).unwrap().to_owned(),
            Group: ints.get(1).unwrap().to_owned(),
            Others: ints.get(2).unwrap().to_owned(),
        }
    };
}

struct FilePerm {
    Owner: u32,
    Group: u32,
    Others: u32,
}
