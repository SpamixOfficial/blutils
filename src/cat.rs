use std::env::args;
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
    // TODO
    #[arg(
        short = 'A',
        long = "show-all",
        help = "equivalent to -vET, indicate all non-printed characters"
    )]
    show_all: bool,
    // Done
    #[arg(
        short = 'b',
        long = "number-nonblank",
        help = "number nonempty output lines, overrides -n"
    )]
    number_nonblank: bool,
    // TODO
    #[arg(short = 'e', help = "equivalent to -vE")]
    show_end_nonprinting: bool,
    // TODO
    #[arg(
        short = 'E',
        long = "show-ends",
        help = "display $ at end of each line"
    )]
    show_ends: bool,
    // Done
    #[arg(short = 'n', long = "number", help = "number all output lines")]
    number: bool,
    // TODO
    #[arg(
        short = 's',
        long = "squeeze-blank",
        help = "suppress repeated empty output lines"
    )]
    squeeze_blank: bool,
    // TODO
    #[arg(short = 't', help = "equivalent to -vT")]
    show_tabs_nonprinting: bool,
    // TODO
    #[arg(short = 'T', long = "show-tabs", help = "display TAB characters as ^I")]
    show_tabs: bool,
    // TODO
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
    if args().collect::<Vec<String>>()[0].split("/").last().unwrap() == "blutils" {
        cli = Cli::parse_from(args().skip(1));
    } else {
        cli = Cli::parse();
    };

    for val in &cli.files {
        let path = Path::new(val);
        // TODO: Get actual error handling working!
        // I know this is a long, long, long line, but here's what is does:
        //
        // It first reads the file from the path provided, then it does some very bad error
        // handling. 
        // After that is splits (inclusively) by newlines, maps every item to owned strings and
        // then it collects it as a Vec<String>
        //
        // Done!
        let mut contents = fs::read_to_string(path)
            .expect("Uh oh! Reading the file went VERY wrong. Report this bug!").split_inclusive('\n').map(|f| f.to_string()).collect::<Vec<String>>();
        if cli.number && cli.number_nonblank != true {
            for (i, line) in contents.clone().iter().enumerate() {
                // Padding is done by the total length of all lines
                contents[i] = format!(" {:<numPadding$} | {line}", i, numPadding = contents.len().to_string().len());
            };
            
        } else if cli.number_nonblank {
            let mut i = 0;
            for (i2, line_string) in contents.clone().iter().enumerate() {
                // Padding is done by the total length of all lines
                if line_string.clone().trim().is_empty() {
                    contents[i2] = format!(" {:<numPadding$} | {line_string}", "", numPadding = contents.len().to_string().len());
                } else {
                    contents[i2] = format!(" {:<numPadding$} | {line_string}", i, numPadding = contents.len().to_string().len());
                    i += 1;
                }
            };            
        };
        println!("{}", contents.iter().map(|f| f.to_owned()).collect::<String>())
    }
    dbg!(&cli);
}
