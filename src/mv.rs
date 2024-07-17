
use std::env::args;

use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(
    version,
    about = "Move (and rename!) files and directories",
    author = "Alexander Hübner"
)]
struct Cli {
    #[clap(value_parser, num_args = 1.., value_delimiter = ' ', required = true)]
    files: Vec<String>,
    #[arg(
        short = 'A',
        long = "show-all",
        help = "equivalent to -vET, indicate all non-printed characters"
    )]
    show_all: bool,
    #[arg(
        short = 'b',
        long = "number-nonblank",
        help = "number nonempty output lines, overrides -n"
    )]
    number_nonblank: bool,
    #[arg(short = 'e', help = "equivalent to -vE")]
    show_end_nonprinting: bool,
    #[arg(
        short = 'E',
        long = "show-ends",
        help = "display $ at end of each line"
    )]
    show_ends: bool,
    #[arg(short = 'n', long = "number", help = "number all output lines")]
    number: bool,
    #[arg(
        short = 's',
        long = "squeeze-blank",
        help = "suppress repeated empty output lines"
    )]
    squeeze_blank: bool,
    #[arg(short = 't', help = "equivalent to -vT")]
    show_tabs_nonprinting: bool,
    #[arg(short = 'T', long = "show-tabs", help = "display TAB characters as ^I")]
    show_tabs: bool,
    #[arg(
        short = 'v',
        long = "show-nonprinting",
        help = "use ^ and M- notation, except for LFD and TAB"
    )]
    show_nonprinting: bool,
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
