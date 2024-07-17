use std::env::args;
use std::ffi::OsStr;
use std::fs;
use std::io::{stdin, Read};
use std::path::Path;
use std::process::exit;

/* Syntax highlighting */
use syntect::easy::HighlightLines;
use syntect::highlighting::{Color, Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(
    version,
    about = "Concatenate FILE(s) to standard output\nWhen \"-\" is passed as a FILE, cat will read from stdin",
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
    #[arg(
        short = 'H',
        long = "highlight",
        help = "Syntax highlight the output file!"
    )]
    highlight: bool,
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
    let files: Vec<String>;
    if cli.files.is_empty() {
        files = vec![String::from("-")]
    } else {
        files = cli.files.clone()
    };
    for val in &files {
        let mut contents;
        if val != "-" {
            let path = Path::new(val);
            contents = match fs::read_to_string(path) {
                Ok(val) => val,
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
            };
        } else {
            let mut stdin = stdin();
            let mut buf: Vec<u8> = vec![];
            match stdin.read_to_end(&mut buf) {
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
            contents = String::from_utf8(buf)
                .expect("This is a bug that shouldnt be possible. Please report this now.");
        };

        let extension = Path::new(val).extension();
        contents = highlight(&cli, contents, extension);
        contents = nonprinting(&cli, contents);
        contents = squeeze_blank(&cli, contents);
        contents = ends(&cli, contents);
        contents = tabs(&cli, contents);
        contents = numbering(&cli, contents);

        println!("{}", contents)
    }
}

fn highlight(cli: &Cli, contents: String, ext: Option<&OsStr>) -> String {
    let mut result = String::from("");

    // Skip if there's no extension

    if !cli.highlight {
        return contents;
    };

    let extension = match ext {
        Some(val) => val.to_str().unwrap(),
        None => return contents,
    };

    // Copy paste from the docs, except for the fact that the extension is dynamic
    // And that I process it in another way
    //
    //
    // So kind of my own thing?
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    // TODO: Fix color background
    let theme = ts.themes["base16-ocean.dark"].clone();

    let syntax = ps.find_syntax_by_extension(extension).unwrap();
    let mut h = HighlightLines::new(syntax, &theme);

    for line in LinesWithEndings::from(contents.as_str()) {
        let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ps).unwrap();
        let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
        result.push_str(escaped.as_str());
    }

    result
}

fn ends(cli: &Cli, contents: String) -> String {
    let mut result: String = contents;
    if cli.show_ends || cli.show_end_nonprinting || cli.show_all {
        result = result
            .clone()
            .split('\n')
            .map(|f| format!("{f}$\n"))
            .collect::<String>();
    }
    result
}

fn squeeze_blank(cli: &Cli, contents: String) -> String {
    let mut result = String::new();
    if cli.squeeze_blank {
        let content_lines = contents
            .clone()
            .split_inclusive('\n')
            .map(|f| f.to_string())
            .collect::<Vec<String>>();
        for line in content_lines {
            // Here we make sure that ALL bytes are counted, not just characters deemed okay by
            // len()
            if line.trim().chars().count() != 0 {
                result.push_str(&line);
            }
        }
    } else {
        result = contents
    }
    return result;
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
                "{} {:<numPadding$} | {line}",
                if cli.highlight { "\x1b[0m" } else { "" },
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
                    "{} {:<numPadding$} | {line_string}",
                    if cli.highlight { "\x1b[0m" } else { "" },
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
    if cli.show_tabs || cli.show_tabs_nonprinting || cli.show_all {
        result = result.replace("\t", "^I");
    }
    return result;
}

fn nonprinting(cli: &Cli, contents: String) -> String {
    // I figured the easiest way here would be to basically just build a new string from chars
    let mut result = String::new();
    if !cli.show_nonprinting
        && !cli.show_end_nonprinting
        && !cli.show_tabs_nonprinting
        && !cli.show_all
    {
        return contents;
    };
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
