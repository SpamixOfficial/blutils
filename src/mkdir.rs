use std::{
    env::args,
    fs::{create_dir, create_dir_all},
    path::{Path, PathBuf},
    process::exit,
    string,
};

use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(
    version,
    about = "Concatenate FILE(s) to standard output\nWhen \"-\" is passed as a FILE, cat will read from stdin",
    author = "Alexander Hübner"
)]
struct Cli {
    // Done
    #[clap(value_parser, num_args = 1.., value_delimiter = ' ', required = true)]
    directories: Vec<PathBuf>,
    // TODO
    #[arg(
        short = 'm',
        long = "mode",
        help = "set file mode (as in chmod), not a=rwx - umask"
    )]
    mode: Option<String>,
    // Done
    #[arg(
        short = 'p',
        long = "parents",
        help = "no error if existing, make parent directories as needed,"
    )]
    parents: bool,
    // Done
    #[arg(
        short = 'v',
        long = "verbose",
        help = "print a message for each created directory"
    )]
    verbose: bool,
    // TODO
    #[arg(
        short = 'Z',
        help = "set SELinux security context of each created directory to the default type"
    )]
    default_selinux: bool,
    // TODO
    #[arg(
        long = "context",
        help = "like -Z, or if CTX is specified then set the SELinux or SMACK security context to CTX"
    )]
    context: Option<String>,
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
        create(&cli, p)
    }
}

fn log(verbose: bool, message: String) {
    if verbose {
        println!("{}", message)
    }
}

fn create(cli: &Cli, path: &PathBuf) {
    log(cli.verbose, format!("Trying to create directory {}", path.display()));  
    if cli.parents {
        log(cli.verbose, String::from("-p flag used, creating parents..."));
        match create_dir_all(path) {
            Err(e) => {
                let mut error_code = 1;
                if let Some(os_error) = e.raw_os_error() {
                    eprintln!("cat: Error: {}", e.to_string());
                    error_code = os_error;
                } else {
                    eprintln!("cat: Error: {}", e.to_string())
                };
                exit(error_code);
            }
            _ => (),
        }
    } else {
        match create_dir_all(path) {
            Err(e) => {
                let mut error_code = 1;
                if let Some(os_error) = e.raw_os_error() {
                    eprintln!("cat: Error: {}", e.to_string());
                    error_code = os_error;
                } else {
                    eprintln!("cat: Error: {}", e.to_string())
                };
                exit(error_code);
            }
            _ => (),
        }
    };
    log(cli.verbose, String::from("Directory was created successfully!"));
}
