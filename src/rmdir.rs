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
    #[clap(value_parser, num_args = 1.., value_delimiter = ' ', required = true)]
    directories: Vec<PathBuf>,
    #[arg(
        long = "ignore-fail-on-non-empty",
        help = "Ignore each failure to remove a non-empty directory"
    )]
    ignore_non_empty: bool,
    #[arg(short = 'm', long = "mute", help = "Won't produce logs of any sort")]
    mute: bool,
    #[arg(
        short = 'p',
        long = "parents",
        help = "Remove DIRECTORY and its ancestors (rmdir -p a/b == rmdir a/b a)"
    )]
    parents: bool,
    #[arg(
        short = 'v',
        long = "verbose",
        help = "Print a message for each created directory"
    )]
    verbose: bool,
}

pub fn main() {
    let cli: Cli;
    let mut exit_code = 0;
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
        exit_code = remove(&cli, p, exit_code);
    }
    exit(exit_code);
}

fn remove(cli: &Cli, path: &PathBuf, exit_code: i32) -> i32 {
    let mut return_code = exit_code;
    log(
        cli.verbose && !cli.mute,
        format!("Trying to remove directory {}", path.display()),
    );
    if cli.parents {
        log(
            cli.verbose && !cli.mute,
            String::from("-p flag used, removing parents..."),
        );
        for p in path.ancestors() {
            if p.display().to_string().is_empty() {
                break;
            };
            log(
                cli.verbose && !cli.mute,
                format!("Removing {}", p.display()),
            );
            match remove_dir(p) {
                Err(e) => {
                    let mut error_code = 1;
                    if let Some(os_error) = e.raw_os_error() {
                        // Here we check for the ignore_non_empty arg
                        // If the first if statement fails we know the arg is active
                        //
                        // Therefor we check for the os_code. If it is "Directory not empty" we
                        // skip printing all together
                        if !cli.ignore_non_empty {
                            eprintln!("rmdir: Error: {}\nTrigger: {}", e.to_string(), p.display());
                        } else if os_error != 39 {
                            eprintln!("rmdir: Error: {}\nTrigger: {}", e.to_string(), p.display());
                        }
                        error_code = os_error;
                    } else {
                        eprintln!("rmdir: Error: {}\nTrigger: {}", e.to_string(), p.display())
                    };

                    return_code = error_code;

                    if !cli.mute {
                        exit(error_code);
                    }
                }
                _ => (),
            }
        }
    } else {
        match remove_dir(path) {
            Err(e) => {
                let mut error_code = 1;
                if let Some(os_error) = e.raw_os_error() {
                    // Read comment on line 79
                    if !cli.ignore_non_empty {
                        eprintln!("rmdir: Error: {}", e.to_string());
                    } else if os_error != 2 {
                        eprintln!("rmdir: Error: {}", e.to_string());
                    }
                    error_code = os_error;
                } else {
                    eprintln!("rmdir: Error: {}", e.to_string())
                };

                return_code = error_code;

                if !cli.mute {
                    exit(error_code);
                }
            }
            _ => (),
        }
    };
    log(
        cli.verbose && !cli.mute,
        format!("Removal of {} successful!", path.display()),
    );
    return_code
}
