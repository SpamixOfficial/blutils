use std::path::PathBuf;

use clap::Parser;

// Read the clap documentation!
#[derive(Parser, Debug, Clone)]
#[command(
    version,
    about = "A description!",
    author = "Alexander Hübner"
)]
struct Cli {
    #[clap(value_parser, num_args = 1.., value_delimiter = ' ', required = true)]
    argument: PathBuf,

    // Once again read the CLAP documentation
    #[arg(
        short = 'o',
        long = "option",
        help = "This is an option"
    )]
    option: Option<String>,
}

pub fn main() {
    let cli: Cli;
    // Standard snippet that needs to be included to make everything work as expected
    // DO NOT TOUCH!!
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
    // Now do something here
    do_something(&cli, &cli.argument);
}

// Always use these arguments if possible
fn do_something(cli: &Cli, path: &PathBuf) {
    return;
}
