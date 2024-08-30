use core::fmt;
use nix::sys::stat::stat;
use std::{
    env::args,
    ffi::OsString,
    fmt::Debug,
    fs::Metadata,
    os::unix::fs::MetadataExt,
    path::{self, Path, PathBuf},
    process::exit,
    usize,
};

use crate::utils::{c_escape, log, ModeWrapper, PathExtras, PathType, PermissionsPlus};

use ansi_term::{Colour, Style};
use chrono::{DateTime, Local, TimeZone};

use clap::Parser;
use regex::Regex;
use walkdir::WalkDir;

const PROGRAM: &str = "ls";

#[derive(Parser, Debug, Clone)]
#[command(
    version,
    about = "List files in source!",
    author = "Alexander Hübner",
    disable_help_flag = true
)]
struct Cli {
    #[clap(value_parser, value_name("FILE"), default_values = ["."])]
    files: Vec<PathBuf>,

    //Done
    #[arg(
        short = 'a',
        long = "all",
        help = "Do not ignore entries starting with ."
    )]
    all: bool,
    //Done
    #[arg(
        short = 'A',
        long = "almost-all",
        help = "Same as all but does not list . and .."
    )]
    almost_all: bool,

    // Done
    #[arg(long = "author", help = "With -l, print the author of each file")]
    author: bool,
    // Done
    #[arg(
        short = 'b',
        long = "escape",
        help = "Print C-style escapes for nongraphic characters"
    )]
    print_escapes: bool,
    // Done
    #[arg(
        long = "block-size",
        help = "With -l, scale sizes by SIZE when printing them; e.g., '--block-size=M'; see SIZE format below"
    )]
    block_size: Option<BlockSize>,
    // Done
    #[arg(
        short = 'B',
        long = "ignore-backups",
        help = "Do not list entries ending with ~ or a specified suffix",
        value_name("suffix"),
        num_args=0..=1,
        default_missing_value("~")
    )]
    ignore_backups: Option<String>,
    // Done
    #[arg(
        short = 'c',
        help = "With -lt: sort by, and show, ctime (time of last change of file status information); with -l: show ctime and sort by name; otherwise: sort by ctime, newest first"
    )]
    sort_access_ctime: bool,
    // I could simply not get this to work, so now lines is the default. Also it looks better (and
    // makes wayyyy more sense!)
    //#[arg(short = 'C', help = "List entries by columns", default_value("true"))]
    //column: bool,
    #[arg(
        long = "color",
        help = "Color the output WHEN",
        default_value("always")
    )]
    color: Option<When>,
    // Done
    #[arg(
        short = 'd',
        long = "directory",
        help = "List directories themselves, not their contents"
    )]
    directory: bool,
    // Not planned... TODO
    /*#[arg(
        short = 'D',
        long = "dired",
        help = "Generate output designed for Emacs' dired mode"
    )]
    dired: bool,*/
    // Done
    #[arg(short = 'f', help = "Do not sort, enable -aU, disable -ls --color")]
    no_sort_color: bool,
    // Done
    #[arg(
        short = 'F',
        long = "classify",
        help = "Append indicator (one of */=>@|) to entries WHEN",
        default_value("always")
    )]
    classify: Option<When>,
    // Done
    #[arg(long = "file-type", help = "Likewise, except do not append '*'", num_args=0..=1, default_missing_value("always"))]
    file_type: Option<When>,
    // TODO
    #[arg(
        long = "format",
        help = "Across -x, commas -m, horizontal -x, long -l, single-column -1, verbose -l, vertical -C"
    )]
    format: Option<FormatWord>,
    // Not planned, might happen in future, TODO
    /*#[arg(long = "full-time", help = "Like -l  --time-style=full-iso")]
    alias_list_time_full_iso: bool,*/
    // Done
    #[arg(short = 'g', help = "Like -l but does not list owner")]
    list_no_owner: bool,
    // Done
    #[arg(
        long = "group-directories-first",
        help = "Group directories before files; can be augmented with a --sort option, but any use of --sort=none (-U) disables grouping"
    )]
    group_directories_first: bool,
    // Done
    #[arg(
        short = 'G',
        long = "no-group",
        help = "In a long listing, dont print group names"
    )]
    no_group: bool,
    // TODO
    #[arg(
        short = 'h',
        long = "human-readable",
        help = "With -l and -s, print sizes like 1K 234M 2G etc."
    )]
    human_readable: bool,
    // TODO
    #[arg(
        long = "si",
        help = "Like human-readable but use powers of 1000, not 1024"
    )]
    human_readable_1000: bool,
    // TODO
    #[arg(
        short = 'H',
        long = "dereference-command-line",
        help = "Always dereference symbolic links passed as arguments"
    )]
    dereference_argument: bool,
    // TODO
    #[arg(
        long = "dereference-command-line-symlink-to-dir",
        help = "Follow each command line symbolic link that points to a directory"
    )]
    dereference_argument_dir: bool,
    // Done
    #[arg(
        long = "hide",
        help = "Do not list entries which matches regex PATTERN, overriden by -a or -A",
        value_name("PATTERN")
    )]
    hide: Option<String>,
    // TODO
    #[arg(long = "hyperlink", help = "Hyperlink file names WHEN")]
    hyperlink_when: Option<When>,
    // TODO
    #[arg(
        long = "indicator-style",
        help = "Append indicator with style WORD to entry names: none (default), slash (-p), file-type (--file-type), classify (-F)",
        default_value("none")
    )]
    indicator_style: Option<IndicatorWord>,
    // Done
    #[arg(
        short = 'i',
        long = "inode",
        help = "Print the index number of each file"
    )]
    inode: bool,
    // Done
    #[arg(
        short = 'I',
        long = "ignore",
        help = "Do not list entries which matches regex PATTERN",
        value_name("PATTERN")
    )]
    ignore_pattern: Option<String>,
    // TODO
    #[arg(
        short = 'k',
        long = "kibibytes",
        help = "Default to 1024-byte blocks for file system usage; used only with -s and directory totals"
    )]
    kibibytes: bool,
    // TODO
    #[arg(short = 'l', help = "Use a long listing format")]
    list: bool,
    //TODO
    #[arg(
        short = 'L',
        long = "dereference",
        help = "Use dereferenced symbolic link information in result instead of symbolic link itself"
    )]
    dereference: bool,
    // TODO
    #[arg(
        short = 'm',
        help = "Fill width with a comma separated list of entries"
    )]
    fill_comma: bool,
    // TODO
    #[arg(
        short = 'n',
        long = "numeric-uid-grid",
        help = "Like l, but list numeric user and group IDs"
    )]
    numeric_list: bool,
    // Done
    #[arg(
        short = 'N',
        long = "literal",
        help = "Print entry names without quoting (the default)"
    )]
    literal: bool,
    // Done
    #[arg(
        short = 'o',
        help = "Like -l but do not list group information - same as -lG"
    )]
    no_group_list: bool,
    // TODO
    #[arg(short = 'p', help = "Append / to directories")]
    slash: bool,
    // TODO
    #[arg(
        short = 'q',
        long = "hide-control-chars",
        help = "Print ? instead of nongraphic characters"
    )]
    hide_control_chars: bool,
    // TODO
    #[arg(
        long = "show-control-chars",
        help = "Show nongraphic as-is (No special visualization)"
    )]
    show_control_chars: bool,
    // TODO
    #[arg(
        short = 'Q',
        long = "quote-name",
        help = "Enclose entry names in double quotes"
    )]
    quote_name: bool,
    // TODO
    #[arg(
        long = "quoting-style",
        help = "Use  quoting  style WORD for entry names: literal, locale, shell, shell-always, shell-escape, shell-escape-always, c, escape (overrides QUOTING_STYLE environment variable)"
    )]
    quoting_style: Option<QuotingWord>,
    // TODO
    #[arg(short = 'r', long = "reverse", help = "Reverse order while sorting")]
    reverse: bool,
    // TODO
    #[arg(
        short = 'R',
        long = "recursive",
        help = "List subdirectories recursively"
    )]
    recursive: bool,
    // TODO
    #[arg(
        short = 's',
        long = "size",
        help = "Print the allocated size of each file, in blocks"
    )]
    size_blocks: bool,
    // TODO
    #[arg(short = 'S', help = "Sort by file size, largest first")]
    size_sort: bool,
    // TODO
    #[arg(
        long = "sort",
        help = "Sort by WORD instead of name: none (-U), size (-S), time (-t), version (-V), extension (-X), width"
    )]
    sort_word: Option<SortWord>,
    // Done
    #[arg(
        long = "time",
        help = "Select which timestamp used to display or sort; access time (-u): atime, access, use; metadata change time (-c): ctime, status;  modified  time  (default): mtime, modification; birth time: birth, creation;\nWith -l, WORD determines which time to show; with --sort=time, sort by WORD (newest first)",
        value_name("WORD")
    )]
    time_display_sort: Option<TimeWord>,
    // Not planned, might happen in the future... TODO
    /*#[arg(
        long = "time-style",
        help = "Time/Date format of -l; TIME_STYLE syntax: {TODO}",
        value_name("TIME_STYLE")
    )]
    time_style: Option<String>,*/
    // done
    #[arg(short = 't', help = "Sort by time")]
    time_sort: bool,
    // TODO
    #[arg(
        short = 'T',
        long = "tabsize",
        help = "Assume tabs stop at each COLS instead of 8",
        value_name("COLS")
    )]
    tab_size: Option<u32>,
    // Done
    #[arg(
        short = 'u',
        help = "With -lt: sort by, and show, access time; with -l: show access time and sort by name; otherwise: sort by access time, newest first"
    )]
    sort_access_time: bool,
    // Done
    #[arg(short = 'U', help = "Do not sort; list entries in directory order")]
    no_sort: bool,
    // TODO
    #[arg(short = 'v', help = "Natural sort of (version) numbers within text")]
    sort_version: bool,
    // TODO
    #[arg(
        short = 'w',
        long = "width",
        help = "Set output width to COLS, 0 means no limit",
        value_name("COLS")
    )]
    output_width: Option<u32>,
    // Not needed as its the default
    //#[arg(short = 'x', help = "List entries by lines instead of columns")]
    //list_lines: bool,
    // TODO
    #[arg(short = 'X', help = "Sort alphabetically by entry extension")]
    sort_extension: bool,
    // Done
    #[arg(long = "zero", help = "End each output line with NUL, not newline")]
    end_nul: bool,
    // Done
    #[arg(short = '1', help = "List one file per line")]
    one_line: bool,
    // Planned for later updates
    //#[arg(long = "update", help = "Control which existing files are updated")]
    //update: Option<Update>,
    #[arg(long = "verbose", help = "explain whats being done")]
    verbose: bool,
    #[clap(long, action = clap::ArgAction::HelpLong)]
    help: Option<bool>,
}

#[derive(clap::ValueEnum, Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[clap(rename_all = "uppercase")]
#[repr(i64)]
enum BlockSize {
    /// KiB (1024)
    K = 1024,
    /// MiB (1024^2)
    M = i64::pow(1024, 2),
    /// GiB (1024^3)
    G = i64::pow(1024, 3),
    /// TiB (1024^4)
    T = i64::pow(1024, 4),
    /// PiB (1024^5)
    P = i64::pow(1024, 5),
    /// EiB (1024^6)
    E = i64::pow(1024, 6),

    // These might be implemented in the future if a need is necessary. At the moment these numbers
    // are way too big!
    /// ZiB (1024^7)
    /*Z = i64::pow(1024, 7),
    /// YiB (1024^8)
    Y = i64::pow(1024, 8),
    /// RiB (1024^9)
    R = i64::pow(1024, 9),
    /// QiB (1024^10)
    Q = i64::pow(1024, 10),*/
    /// KB (1000)
    KB = 1000,
    /// MB (1000^2)
    MB = i64::pow(1000, 2),
    /// GB (1000^3)
    GB = i64::pow(1000, 3),
    /// TB (1000^4)
    TB = i64::pow(1000, 4),
    /// PB (1000^5)
    PB = i64::pow(1000, 5),
    /// EB (1000^6)
    EB = i64::pow(1000, 6),
    /*
    // Same thing here
    /// ZB (1000^7)
    ZB = 1000 ^ 7,
    /// YB (1000^8)
    YB = 1000 ^ 8,
    /// RB (1000^9)
    RB = 1000 ^ 9,
    /// QB (1000^10)
    QB = 1000 ^ 10,
    */
}

#[derive(clap::ValueEnum, Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
enum When {
    #[default]
    Always,
    Auto,
    Never,
}

impl When {
    fn to_i8(&self) -> i8 {
        match self {
            When::Always => 0,
            When::Auto => 1,
            When::Never => 2,
        }
    }
}

#[derive(clap::ValueEnum, Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum FormatWord {
    Across,
    Commas,
    Horizontal,
    Long,
    SingleColumn,
    Verbose,
    Vertical,
}

#[derive(clap::ValueEnum, Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum QuotingWord {
    Literal,
    Locale,
    Shell,
    ShellAlways,
    ShellEscape,
    ShellEscapeAlways,
    C,
    Escape,
}

#[derive(clap::ValueEnum, Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum IndicatorWord {
    None,
    Slash,
    FileType,
    Classify,
}

#[derive(clap::ValueEnum, Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum SortWord {
    None,
    Size,
    Time,
    Version,
    Extension,
    Width,
}

#[derive(clap::ValueEnum, Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum TimeWord {
    #[value(name = "atime", aliases(["access", "use"]))]
    AccessTime,
    #[value(name = "ctime", aliases(["status"]))]
    MetadataChangeTime,
    #[value(name = "mtime", aliases(["modification"]))]
    ModifiedTime,
}

#[derive(Debug, Clone)]
struct EntryItem {
    parent: PathBuf,
    mode: ModeWrapper,
    number_of_entries: usize,
    owner: String,
    group: String,
    size: usize,
    size_char: String,
    timestamps: DisplayTime,
    processed_entry: String,
    metadata_entry: Metadata,
    inode: u64,
    author: String,
}

#[derive(Debug, Clone)]
struct Longest {
    number_of_entries: usize,
    longest_owner: usize,
    longest_group: usize,
    longest_size: usize,
    longest_inode: usize,
    longest_author: usize,
}

pub fn main() {
    let mut cli: Cli;
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

    if cli.time_sort {
        cli.sort_word = Some(SortWord::Time);
    }
    if cli.sort_access_time {
        cli.time_display_sort = Some(TimeWord::AccessTime)
    }
    if cli.sort_access_ctime {
        cli.time_display_sort = Some(TimeWord::MetadataChangeTime)
    }
    if cli.hide.is_some() && cli.ignore_pattern.is_none() && !(cli.all || cli.almost_all) {
        cli.ignore_pattern = Some(cli.hide.clone().unwrap())
    }
    if cli.no_group_list {
        cli.list = true;
        cli.no_group = true;
    }

    if cli.file_type.is_some() {
        cli.classify = cli.file_type
    }

    if cli.no_sort_color {
        cli.all = true;
        cli.no_sort = true;
        cli.list = false;
        cli.size_blocks = false;
        cli.color = Some(When::Never);
    }

    for file in &cli.files {
        if !file.exists() {
            eprintln!("No such file or directory: {}", file.display());
            continue;
        }
        ls(&cli, file);
    }
}

fn ls(cli: &Cli, p: &PathBuf) {
    let mut dir = WalkDir::new(p).min_depth(1);
    if !cli.recursive {
        dir = dir.max_depth(1)
    }

    // First we get and collect all the entries into a vector of strings
    let entries: Vec<(String, PathBuf, usize)>;
    if p.is_dir() && !cli.directory {
        entries = dir
            .into_iter()
            .filter_map(|e| e.ok())
            .map(|e| {
                (
                    e.clone()
                        .into_path()
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string(),
                    e.clone().into_path(),
                    e.depth(),
                )
            })
            .collect();
    } else {
        // Manually create an one item vector if not a dir
        entries = vec![(p.to_str().unwrap().to_string(), p.to_owned(), 0)]
    }
    // Create the lines variable we will use later
    // Also get longest entry because the processing introduces asci control characters in most
    // cases!
    let (lines, longest_entry) = treat_entries(cli, entries);

    // Finally trigger the right function
    //if !cli.recursive {
    if !cli.list {
        normal_list(cli, lines, longest_entry);
    } else {
        list_list(cli, lines)
    }
    /*} else {
        recursive_list(cli, lines, longest_entry);
    }*/
}

fn treat_entries(
    cli: &Cli,
    entries_list: Vec<(String, PathBuf, usize)>,
) -> (Vec<Vec<(String, PathBuf, usize, usize)>>, usize) {
    let term_size = termsize::get();
    let mut entries: Vec<(String, PathBuf, usize, usize)> = entries_list
        .into_iter()
        .map(|f| (f.0.clone(), f.1, f.0.len(), f.2))
        .collect();

    // Here we remove and add items as needed
    if let Some(suffix) = &cli.ignore_backups {
        entries.retain(|x| x.0.ends_with(suffix.as_str()) != true);
    }

    if let Some(pattern) = &cli.ignore_pattern {
        let re = match Regex::new(pattern) {
            Ok(x) => x,
            Err(e) => {
                eprintln!(
                    "Supplied PATTERN was not a valid regex pattern: {}",
                    e.to_string()
                );
                exit(1);
            }
        };

        entries.retain(|x| re.is_match(x.0.as_str()) != true);
    }

    // Here we start treating the vector and variables
    // Sorting
    if cli.sort_word.is_none() {
        entries.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));
    } else if let Some(word) = cli.sort_word {
        match word {
            SortWord::Time => {
                let time_word = if let Some(x) = cli.time_display_sort {
                    x
                } else {
                    TimeWord::ModifiedTime
                };
                entries.sort_by(|a, b| {
                    let timestamps = (
                        FileTimestamps::new(a.1.clone()),
                        FileTimestamps::new(b.1.clone()),
                    );
                    let times = match time_word {
                        TimeWord::ModifiedTime => (timestamps.0.modified, timestamps.1.modified),
                        TimeWord::AccessTime => (timestamps.0.access, timestamps.1.access),
                        TimeWord::MetadataChangeTime => {
                            (timestamps.0.metadata_change, timestamps.1.metadata_change)
                        }
                    };
                    times.1.unix.cmp(&times.0.unix)
                })
            }
            SortWord::None => (),
            _ => (),
        }
    }

    if cli.group_directories_first && cli.sort_word != Some(SortWord::None) {
        entries.sort_by(|a, b| b.1.is_dir().cmp(&a.1.is_dir()));
    }

    // If the all and almost all mode isn't activated we need to do some filtering
    if !cli.almost_all && !cli.all {
        entries = entries
            .into_iter()
            .filter_map(|f| if !f.0.starts_with(".") { Some(f) } else { None })
            .collect();
    } else if cli.all {
        entries.insert(0, (String::from("."), PathBuf::from("./"), 1, 1));
        entries.insert(1, (String::from(".."), PathBuf::from("../"), 1, 2));
    }

    // C style escaping. We put this before the color so it doesnt start escaping the color codes!
    if cli.print_escapes {
        entries = entries
            .into_iter()
            .map(|entry| (c_escape(entry.0, false), entry.1, entry.2, entry.3))
            .collect();
    }

    // Sort so depth makes sense in case of recursive
    if cli.recursive {
        entries.sort_by(|a, b| a.3.cmp(&b.3));
    }

    // If no terminal size we can assume it was called either as a background process or some other
    // non-graphical process
    if term_size.is_none() {
        entries
            .iter()
            .for_each(|entry| print!("{}{}", entry.0, if cli.end_nul { "\0" } else { "\n" }));
        exit(0);
    }

    if cli.color == Some(When::Always)
        || cli.color.is_none()
        || (cli.color == Some(When::Auto) && term_size.is_some())
    {
        entries = entries
            .into_iter()
            .map(|entry| {
                let style = match entry.1.ptype() {
                    PathType::Directory => Style::new().bold().fg(Colour::Blue),
                    PathType::Executable => Style::new().bold().fg(Colour::Green),
                    PathType::Symlink => Style::new().bold().fg(Colour::Cyan),
                    _ => Style::new(),
                };
                (style.paint(entry.0).to_string(), entry.1, entry.2, entry.3)
            })
            .collect();
    }

    // We start splitting up here
    // If no terminal size we can assume it was called either as a background process or some other
    // non-graphical process
    if term_size.is_none() {
        entries.iter().for_each(|entry| println!("{}", entry.0));
        exit(0);
    }

    let mut longest_entry = entries
        .iter()
        .map(|x| x.0.len() + 2 + x.0.len() - x.2)
        .max()
        .unwrap_or(0);
    // Here we can safely assume that the above function failed because the entries list is too
    // short, so we simply set it ourselves
    if longest_entry == 0 && entries.len() == 1 {
        longest_entry = entries.get(0).unwrap().0.len();
    }
    // Get the maximum entries per line and use this to create a new Vec<Vec<String>>
    let entry_per_line = term_size.unwrap().cols as usize / (longest_entry);
    //if cli.list_lines {
    (
        entries
            .chunks(entry_per_line)
            .map(|s| s.into())
            .collect::<Vec<Vec<(String, PathBuf, usize, usize)>>>(),
        longest_entry,
    )
    /*} else {
        let mut chunk_size = entries.len() / entry_per_line;
        if chunk_size < 2 {
            chunk_size = entries.len()
        };
        (
            entries
                .chunks(chunk_size)
                .map(|s| s.into())
                .collect::<Vec<Vec<(String, PathBuf, usize)>>>(),
            longest_entry,
        )
    }*/
}

fn recursive_list(
    cli: &Cli,
    lines: Vec<Vec<(String, PathBuf, usize, usize)>>,
    longest_entry: usize,
) {
}

fn normal_list(cli: &Cli, lines: Vec<Vec<(String, PathBuf, usize, usize)>>, longest_entry: usize) {
    let mut current_dir = PathBuf::new();
    if cli.one_line {
        lines.iter().for_each(|entries| {
            for entry in entries {
                print!("{}", entry.0);
                print!("{}", if cli.end_nul { "\0" } else { "\n" })
            }
        });
        exit(0);
    }
    if lines.len() > 2 {
        for line in lines {
            for entry in line {
                if cli.recursive && entry.1.parent() != Some(current_dir.as_path()) {
                    println!(
                        "\n{}:",
                        entry.1.parent().unwrap_or(&path::Path::new("./")).display()
                    );
                    current_dir = entry.1.parent().unwrap().to_path_buf();
                }
                /*let style = match entry.1.as_path().ptype() {
                    PathType::Directory => Style::new().bold().fg(Colour::Blue),
                    PathType::Executable => Style::new().bold().fg(Colour::Green),
                    PathType::Symlink => Style::new().bold().fg(Colour::Cyan),
                    _ => Style::new(),
                };*/
                let entry_format_string = format!(
                    "{: <width$}",
                    entry.0.clone()
                        + entry
                            .1
                            .str_classify(
                                cli.file_type.is_some(),
                                cli.classify.unwrap_or_default().to_i8()
                            )
                            .as_str(),
                    width = longest_entry + 1 + (entry.0.len() - entry.2)
                );
                print!("{}", entry_format_string);
            }
            print!("{}", if cli.end_nul { "\0" } else { "\n" })
        }
    /*} else if lines.len() > 1 {
    let entries = lines.clone().get(1).unwrap().len();
    for i in 0..entries - 1 {
        for (i2, line) in lines.iter().enumerate() {
            let entry = if let Some(x) = line.get(i) {
                x
            } else {
                continue;
            };
            let entry_format_string = format!(
                "{: <width$}",
                entry.0.clone()
                    + match entry.1.as_path().ptype() {
                        PathType::Symlink => "@",
                        PathType::Directory => "/",
                        PathType::Executable => "*",
                        _ => "",
                    },
                // Doing witchcraft here to make sure formatting looks nice!
                width = if i2 != lines.len() - 1 {
                    longest_entry + 2 + (entry.0.len() - entry.2)
                } else {
                    0
                }
            );
            print!("{}", entry_format_string);
        }
        print!("\n");
    }*/
    } else {
        for line in lines {
            for entry in line {
                if cli.recursive && entry.1.parent() != Some(current_dir.as_path()) {
                    println!(
                        "\n{}:",
                        entry.1.parent().unwrap_or(&path::Path::new("./")).display()
                    );
                    current_dir = entry.1.parent().unwrap().to_path_buf();
                }
                print!(
                    "{}{}  ",
                    entry.0,
                    entry
                        .1
                        .str_classify(
                            cli.file_type.is_some(),
                            cli.classify.unwrap_or_default().to_i8()
                        )
                        .as_str()
                );
            }
            print!("{}", if cli.end_nul { "\0" } else { "\n" })
        }
    }
}

fn list_list(cli: &Cli, lines: Vec<Vec<(String, PathBuf, usize, usize)>>) {
    let mut entries: Vec<EntryItem> = vec![];
    let mut current_dir = PathBuf::new();
    for line in lines {
        for entry in line {
            let style = match entry.1.ptype() {
                PathType::Directory => Style::new().bold().fg(Colour::Blue),
                PathType::Executable => Style::new().bold().fg(Colour::Green),
                PathType::Symlink => Style::new().bold().fg(Colour::Cyan),
                _ => Style::new(),
            };

            let metadata_entry = entry.1.metadata().unwrap();

            // Get the permission string (Example: -rw-r--r--, octal 644)
            let perms = metadata_entry.permissions().mode_struct();
            // Get entries in directory, or 1 if its a file
            let dir_entries = if entry.1.is_dir() {
                WalkDir::new(&entry.1).max_depth(1).into_iter().count()
            } else {
                1
            };
            // Get owner and group
            let owner = if cli.list_no_owner {
                OsString::from("")
            } else {
                users::get_current_username().unwrap_or(OsString::from("unknown"))
            };

            let group = if cli.no_group {
                OsString::from("")
            } else {
                users::get_current_groupname().unwrap_or(OsString::from("unknown"))
            };

            // Create timestamps
            let file_timestamp = FileTimestamps::new(entry.1.clone());

            // Now choose the right time to display
            let timestamp = if let Some(word) = cli.time_display_sort {
                DisplayTime::new(match word {
                    TimeWord::AccessTime => file_timestamp.access,
                    TimeWord::ModifiedTime => file_timestamp.modified,
                    TimeWord::MetadataChangeTime => file_timestamp.metadata_change,
                })
            } else {
                DisplayTime::new(file_timestamp.modified)
            };

            let parent = entry.1.parent().unwrap_or(Path::new("./")).to_path_buf();

            let inode = if cli.inode {
                match stat(&entry.1) {
                    Ok(x) => x.st_ino,
                    Err(e) => {
                        log(
                            cli.verbose,
                            format!("Inode failed for {}: {}", &entry.1.display(), e.to_string()),
                        );
                        0
                    }
                }
            } else {
                0
            };

            let (block_size, size_char) = if let Some(bs) = cli.block_size {
                dbg!(&bs, bs as i64);
                (bs as i64, format!("{:?}", &bs))
            } else {
                (1, String::from(" "))
            };

            dbg!(block_size, metadata_entry.size());

            // Finally create the format string
            let entry_item = EntryItem {
                parent,
                mode: perms,
                number_of_entries: dir_entries,
                owner: owner.to_str().unwrap().to_string(),
                group: group.to_str().unwrap().to_string(),
                size: metadata_entry.size() as usize / block_size as usize,
                size_char,
                timestamps: timestamp,
                processed_entry: style.paint(entry.0.clone()).to_string()
                    + entry
                        .1
                        .str_classify(
                            cli.file_type.is_some(),
                            cli.classify.unwrap_or_default().to_i8(),
                        )
                        .as_str(),
                metadata_entry,
                inode,
                // This is hilarious, but the author is just the owner. Why does this option even
                // exist?
                author: if cli.author {
                    owner.to_str().unwrap().to_string() + " "
                } else {
                    String::from("")
                },
            };
            entries.push(entry_item);
        }
    }

    // All "longest-variables"
    let longest = Longest {
        number_of_entries: entries
            .clone()
            .iter()
            .map(|x| x.number_of_entries.to_string().chars().count())
            .max()
            .unwrap_or(0),
        longest_owner: entries
            .clone()
            .iter()
            .map(|x| x.owner.chars().count())
            .max()
            .unwrap_or(0),
        longest_group: entries
            .clone()
            .iter()
            .map(|x| x.group.chars().count())
            .max()
            .unwrap_or(0),
        longest_size: entries
            .clone()
            .iter()
            .map(|x| x.size.to_string().chars().count())
            .max()
            .unwrap_or(0),
        longest_inode: entries
            .clone()
            .iter()
            .map(|x| x.inode.to_string().chars().count())
            .max()
            .unwrap_or(0),
        longest_author: entries
            .clone()
            .iter()
            .map(|x| x.author.chars().count())
            .max()
            .unwrap_or(0),
    };

    print!("total {}", entries.len());
    print!("{}", if cli.end_nul { "\0" } else { "\n" });
    entries.iter().for_each(|f| {
        // Here we check if the parent matches the current parent, and if not we can safely assume
        // that we have changed dir and should print a new "directory entry summary line"
        //
        // Only for recursive
        if cli.recursive && f.parent != current_dir {
                println!(
                    "\n{}:",
                    f.parent.display()
                );
                current_dir = f.parent.clone();
        };
        println!(
            "{: >longest_inode$} {} {: >longest_dir$} {: >longest_user$} {: >longest_group$} {: >longest_author$}{: >longest_size$}{} {} {} {} {}",
            if cli.inode { format!("{}", f.inode)} else { String::from("") },
            f.mode.to_string(),
            f.number_of_entries,
            f.owner,
            f.group,
            f.author,
            f.size,
            f.size_char,
            f.timestamps.month,
            f.timestamps.date,
            f.timestamps.time,
            f.processed_entry,
            longest_inode = if cli.inode { longest.longest_inode } else {0},
            longest_dir = longest.number_of_entries,
            longest_user = longest.longest_owner,
            longest_group = longest.longest_group,
            longest_size = longest.longest_size,
            longest_author = longest.longest_author
        )
    });
}

#[derive(Debug, Copy, Clone)]
struct FileTimestamps {
    access: Timestamp,
    /*birth: Timestamp,*/ // Ignored for now, will implement when I find a way to do it!
    modified: Timestamp,
    metadata_change: Timestamp,
}

impl FileTimestamps {
    fn new(p: PathBuf) -> FileTimestamps {
        let metadata = p.metadata().unwrap();
        let access = Timestamp {
            unix: metadata.atime(),
            datetime: Local
                .timestamp_opt(metadata.atime(), 0)
                .single()
                .unwrap_or(Local.timestamp_opt(0, 0).unwrap()),
        };
        let modified = Timestamp {
            unix: metadata.mtime(),
            datetime: Local
                .timestamp_opt(metadata.mtime(), 0)
                .single()
                .unwrap_or(Local.timestamp_opt(0, 0).unwrap()),
        };
        let metadata_change = Timestamp {
            unix: metadata.ctime(),
            datetime: Local
                .timestamp_opt(metadata.ctime(), 0)
                .single()
                .unwrap_or(Local.timestamp_opt(0, 0).unwrap()),
        };

        FileTimestamps {
            access,
            modified,
            metadata_change,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Timestamp {
    unix: i64,
    datetime: DateTime<Local>,
}

#[derive(Debug, Clone)]
struct DisplayTime {
    month: String,
    date: String,
    time: String,
    year: String,
}

impl DisplayTime {
    fn new(time: Timestamp) -> DisplayTime {
        let date_naive = time.datetime.date_naive();
        let time_naive = time.datetime.time();
        let month = date_naive.format("%b").to_string().to_lowercase();
        let date = date_naive.format("%e").to_string();
        let time = time_naive.format("%H:%M").to_string();
        let year = date_naive.format("%Y").to_string();
        DisplayTime {
            month,
            date,
            time,
            year,
        }
    }
}
