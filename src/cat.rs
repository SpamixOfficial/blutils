use std::env::args;
use std::path::{Path, PathBuf};
use std::{
    env::{args_os, Args},
    fs,
    str::Lines,
};

use clap::{ArgAction, Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
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
            .expect("Uh oh! Reading the file went VERY wrong. Report this bug!");

        contents = tabs(&cli, contents);
        contents = numbering(&cli, contents);

        println!("{}", contents)
    }
    dbg!(&cli);
}

fn numbering(cli: &Cli, contents: String) -> String {
    // Create a new Vector of strings (lines) from the content lines
    let mut content_lines = contents
        .clone()
        .split_inclusive('\n')
        .map(|f| f.to_string())
        .collect::<Vec<String>>();
    if cli.number && cli.number_nonblank != true {
        for (i, line) in content_lines.clone().iter().enumerate() {
            // Padding is done by the total length of all lines
            content_lines[i] = format!(
                " {:<numPadding$} | {line}",
                i,
                numPadding = content_lines.len().to_string().len()
            );
        }
    } else if cli.number_nonblank {
        let mut i = 0;
        for (i2, line_string) in content_lines.clone().iter().enumerate() {
            // Padding is done by the total length of all lines
            if line_string.clone().trim().is_empty() {
                content_lines[i2] = format!(
                    " {:<numPadding$} | {line_string}",
                    "",
                    numPadding = content_lines.len().to_string().len()
                );
            } else {
                content_lines[i2] = format!(
                    " {:<numPadding$} | {line_string}",
                    i,
                    numPadding = content_lines.len().to_string().len()
                );
                i += 1;
            }
        }
    }
    // Turn lines to string and return
    content_lines
        .iter()
        .map(|f| f.to_owned())
        .collect::<String>()
}

fn tabs(cli: &Cli, contents: String) -> String {
    let mut result = contents;
    if cli.show_tabs && cli.show_tabs_nonprinting != true {
        result = result.replace("\t", "^I");
        dbg!(&result.replace("\t", "^I"));
    } else if cli.show_tabs_nonprinting {
        result = result.replace("\t", "^I");
        result = nonprinting(&cli.clone(), result);
    };
    return result;
}

fn nonprinting(cli: &Cli, contents: String) -> String {
    // I figured the easiest way here would be to basically just build a new string from chars
    let mut result = String::new(); 
    for ch in contents.chars() {
        // Make sure it isnt a control code
        if ch as u8 >= 32 {
            if 127 > ch as u8 {
                // Printable char, just push it to result string
                result.push(ch);
            } else if ch as u8 == 127 {
                // Del char
                result.push_str("^?");
            } else {
                // Meta characters
                result.push_str("M-");
                if 128 + 32 <= ch as u8 {
                    if 128 + 127 > ch as u8 {
                        // Meta character is out of the ascii range, we remove 128 to make it
                        // printable
                        result.push((ch as u8 - 128) as char)
                    } else {
                        result.push_str("^?");
                    }
                } else {
                    result.push('^');
                    result.push((ch as u8 - 128 + 64) as char);
                }
            }
        } else if ch == '\t' && !cli.show_tabs {
            result.push(ch)
        } else if ch == '\n' {
            result.push(ch)
        } else {
            // If it is a control code we push ^, and add 64 to the char to its printable
            result.push('^');
            result.push((ch as u8 + 64) as char);
        }
    }
    result
}
