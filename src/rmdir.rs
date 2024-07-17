use std::{env::args, fs::remove_dir, path::PathBuf, process::exit};

use crate::utils::log;
use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(
    version,
    about = "Remove directories EMPTY directories",
    author = "Alexander Hübner"
)]
struct Cli {
    // Done
    #[clap(value_parser, num_args = 1.., value_delimiter = ' ', required = true)]
    directories: Vec<PathBuf>,
    // TODO
    #[arg(
        long = "ignore-fail-on-non-empty",
        help = "Ignore each failure to remove a non-empty directory"
    )]
    ignore_non_empty: bool,
    // TODO
    #[arg(
        short = 'm',
        long = "mute",
        help = "Won't produce logs of any sort"
    )]
    mute: bool,
    // Done
    #[arg(
        short = 'p',
        long = "parents",
        help = "Remove DIRECTORY and its ancestors (rmdir -p a/b == rmdir a/b a)"
    )]
    parents: bool,
    // Done
    #[arg(
        short = 'v',
        long = "verbose",
        help = "Print a message for each created directory"
    )]
    verbose: bool,
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
    for p in &cli.directories {
        remove(&cli, p);
    }
}

fn remove(cli: &Cli, path: &PathBuf) {
    log(
        cli.verbose,
        format!("Trying to remove directory {}", path.display()),
    );
    if cli.parents {
        log(
            cli.verbose,
            String::from("-p flag used, removing parents..."),
        );
        for p in path.ancestors() {
            if p.display().to_string().is_empty() {
                break
            };
            log(
                cli.verbose,
                format!("Removing {}", p.display()),
            );
            match remove_dir(p) {
                Err(e) => {
                    let mut error_code = 1;
                    if let Some(os_error) = e.raw_os_error() {
                        eprintln!("rmdir: Error: {}\nTrigger: {}", e.to_string(), p.display());
                        error_code = os_error;
                    } else {
                        eprintln!("rmdir: Error: {}\nTrigger: {}", e.to_string(), p.display())
                    };
                    exit(error_code);
                }
                _ => (),
            }
        }
    } else {
        match remove_dir(path) {
            Err(e) => {
                let mut error_code = 1;
                if let Some(os_error) = e.raw_os_error() {
                    eprintln!("rmdir: Error: {}", e.to_string());
                    error_code = os_error;
                } else {
                    eprintln!("rmdir: Error: {}", e.to_string())
                };
                exit(error_code)
            }
            _ => (),
        }
    };
    log(
        cli.verbose,
        format!("Removal of {} successful!", path.display()),
    );
}
