use std::{env::args, fs::create_dir, process::exit, string};

use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(
    version,
    about = "Concatenate FILE(s) to standard output\nWhen \"-\" is passed as a FILE, cat will read from stdin",
    author = "Alexander Hübner"
)]
struct Cli {
    #[clap(value_parser, num_args = 1.., value_delimiter = ' ', required = true)]
    directories: Vec<String>,
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
    #[arg(
        short = 'Z',
        help = "set SELinux security context of each created directory to the default type"
    )]
    default_selinux: bool,
    #[arg(
        long = "context",
        help = "like -Z, or if CTX is specified then set the SELinux or SMACK security context to CTX"
    )]
    number: Option<String>,
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
}
