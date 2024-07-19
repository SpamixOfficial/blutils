use std::{
    env::args,
    fs::{create_dir, create_dir_all, set_permissions, Permissions},
    os::unix::fs::PermissionsExt,
    path::PathBuf,
    process::exit,
};

use crate::utils::log;

use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(
    version,
    about = "Create directories that doesnt already exist!",
    author = "Alexander Hübner"
)]
struct Cli {
    #[clap(value_parser, num_args = 1.., value_delimiter = ' ', required = true)]
    directories: Vec<PathBuf>,

    #[arg(
        short = 'm',
        long = "mode",
        help = "set file mode (as in chmod), not a=rwx - umask"
    )]
    mode: Option<String>,

    #[arg(
        short = 'p',
        long = "parents",
        help = "no error if existing, make parent directories as needed,"
    )]
    parents: bool,

    #[arg(
        short = 'v',
        long = "verbose",
        help = "print a message for each created directory"
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
    // Loop over each directory
    for p in &cli.directories {
        create(&cli, p);
        mode(&cli, p);
    }
}

fn mode(cli: &Cli, path: &PathBuf) {
    if let Some(modestr) = cli.clone().mode {
        log(
            cli.verbose,
            format!("Setting mode {} for {}", modestr, path.display()),
        );
        let mode = match u32::from_str_radix(modestr.as_str(), 8) {
            Ok(val) => val,
            Err(e) => {
                eprintln!("mkdir: Error: {}", e.to_string());
                exit(1)
            }
        };

        match set_permissions(path, Permissions::from_mode(mode)) {
            Err(e) => {
                let mut error_code = 1;
                if let Some(os_error) = e.raw_os_error() {
                    eprintln!("mkdir: Error: {}", e.to_string());
                    error_code = os_error;
                } else {
                    eprintln!("mkdir: Error: {}", e.to_string())
                };
                exit(error_code);
            }
            _ => (),
        };
        log(cli.verbose, "Modeset was successful!");
    };
}

fn create(cli: &Cli, path: &PathBuf) {
    log(
        cli.verbose,
        format!("Trying to create directory {}", path.display()),
    );
    if cli.parents {
        log(
            cli.verbose,
            "-p flag used, creating parents...",
        );
        match create_dir_all(path) {
            Err(e) => {
                let mut error_code = 1;
                if let Some(os_error) = e.raw_os_error() {
                    eprintln!("mkdir: Error: {}", e.to_string());
                    error_code = os_error;
                } else {
                    eprintln!("mkdir: Error: {}", e.to_string())
                };
                exit(error_code);
            }
            _ => (),
        }
    } else {
        match create_dir(path) {
            Err(e) => {
                let mut error_code = 1;
                if let Some(os_error) = e.raw_os_error() {
                    eprintln!("mkdir: Error: {}", e.to_string());
                    error_code = os_error;
                } else {
                    eprintln!("mkdir: Error: {}", e.to_string())
                };
                exit(error_code);
            }
            _ => (),
        }
    };
    log(
        cli.verbose,
        "Directory was created successfully!",
    );
}
