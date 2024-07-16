
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
        long = "ignore-fail-all",
        help = "Ignore EVERY failure, even if the directory doesn't exist. \n\tWill exit with exit code, but wont produce logs. Useful for log-free \"try-catch\" loops."
    )]
    ignore_all: bool,
    // TODO
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
        format!("Trying to create directory {}", path.display()),
    );
    if cli.parents {
        log(
            cli.verbose,
            String::from("-p flag used, creating parents..."),
        );
        match remove_dir(path) {
            Err(e) => {
                let mut error_code = 1;
                if let Some(os_error) = e.raw_os_error() {
                    eprintln!("rmdir: Error: {}", e.to_string());
                    error_code = os_error;
                } else {
                    eprintln!("rmdir: Error: {}", e.to_string())
                };
                exit(error_code);
            }
            _ => (),
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
            },
            _ => ()
        }
    };
}
