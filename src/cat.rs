use std::{env::{args_os, Args}, fs};
use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[clap(value_parser, num_args = 1.., value_delimiter = ' ')]
    files: Vec<String>,
    #[arg(short = 'A', long = "show-all", help = "equivalent to -vET, indicate all non-printed characters")]
    show_all: bool
}

pub fn main() {
    let cli = Cli::parse();
    dbg!(&cli);
}
