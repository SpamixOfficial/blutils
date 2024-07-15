use std::path::{Path, PathBuf};
use std::{
    env::{args_os, Args},
    fs,
    str::Lines,
};

use clap::{ArgAction, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    version,
    about = "Concatenate FILE(s) to standard output\nWhen \"-\" is passed as a FILE, cat will read from stdin",
    author = "Alexander Hübner"
)]
struct Cli {
    #[clap(value_parser, num_args = 1.., value_delimiter = ' ')]
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
    let cli = Cli::parse();
    for val in &cli.files {
        let path = Path::new(val);
        dbg!(&path);
        let mut contents = fs::read_to_string(path)
            .expect("Uh oh! Reading the file went VERY wrong. Report this bug!").split_inclusive('\n').map(|f| f.to_string()).collect::<Vec<String>>();
        if cli.number && !cli.number_nonblank {
            let mut i = 0;
            let padding = 
            for line in contents.clone() {
                contents[i] = format!(" {} | {}", i, line);
                i += 1;
            };
            
        } else if cli.number_nonblank {
        };
        println!("{}", contents.iter().map(|f| f.to_owned()).collect::<String>())
    }
    dbg!(&cli);
}
