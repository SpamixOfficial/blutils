use std::{env::args, ffi::OsString, os::unix::fs::MetadataExt, path::PathBuf, process::exit};

use crate::utils::{PathExtras, PathType, PermissionsPlus};

use ansi_term::{Colour, Style};
use chrono::{DateTime, Utc, NaiveDateTime};

use clap::Parser;
use walkdir::WalkDir;

const PROGRM: &str = "ls";

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

    //TODO
    #[arg(long = "author", help = "With -l, print the author of each file")]
    author: bool,
    //TODO
    #[arg(
        short = 'b',
        long = "escape",
        help = "Print C-style escapes for nongraphic characters"
    )]
    print_escapes: bool,
    #[arg(
        long = "block_size",
        help = "With -l, scale sizes by SIZE when printing them; e.g., '--block-size=M'; see SIZE format below"
    )]
    block_size: Option<BlockSize>,
    // TODO
    #[arg(
        short = 'B',
        long = "ignore-backups",
        help = "Do not list entries ending with ~ or a specified suffix",
        value_name("suffix"),
        default_missing_value("~")
    )]
    ignore_backups: Option<String>,
    #[arg(
        short = 'c',
        help = "With -lt: sort by, and show, ctime (time of last change of file status information); with -l: show ctime and sort by name; otherwise: sort by ctime, newest first"
    )]
    show_ctime: bool,
    #[arg(short = 'C', help = "List entries by columns", default_value("true"))]
    column: bool,
    #[arg(long = "color", help = "Color the output WHEN", default_value("never"))]
    color: Option<When>,
    #[arg(
        short = 'd',
        long = "directory",
        help = "List directories themselves, not their contents"
    )]
    directory: bool,
    #[arg(
        short = 'D',
        long = "dired",
        help = "Generate output designed for Emacs' dired mode"
    )]
    dired: bool,
    #[arg(short = 'f', help = "Do not sort, enable -aU, disable -ls --color")]
    no_sort_color: bool,
    #[arg(
        short = 'F',
        long = "classify",
        help = "Append indicator (one of */=>@|) to entries WHEN"
    )]
    classify: Option<When>,
    #[arg(long = "file-type", help = "Likewise, except do not append '*'")]
    file_type: bool,
    #[arg(
        long = "format",
        help = "Across -x, commas -m, horizontal -x, long -l, single-column -1, verbose -l, vertical -C"
    )]
    format: Option<FormatWord>,
    #[arg(long = "full-time", help = "Like -l  --time-style=full-iso")]
    alias_list_time_full_iso: bool,
    #[arg(short = 'g', help = "Like -l but does not list owner")]
    list_no_owner: bool,
    #[arg(
        long = "group-directories-first",
        help = "Group directories before files; can be augmented with a --sort option, but any use of --sort=none (-U) disables grouping"
    )]
    group_directories_first: bool,
    #[arg(
        short = 'G',
        long = "no-group",
        help = "In a long listing, dont print group names"
    )]
    no_group: bool,
    #[arg(
        short = 'h',
        long = "human-readable",
        help = "With -l and -s, print sizes like 1K 234M 2G etc."
    )]
    human_readable: bool,
    #[arg(
        long = "si",
        help = "Like human-readable but use powers of 1000, not 1024"
    )]
    human_readable_1000: bool,
    #[arg(
        short = 'H',
        long = "dereference-command-line",
        help = "Always dereference symbolic links passed as arguments"
    )]
    dereference_argument: bool,
    #[arg(
        long = "dereference-command-line-symlink-to-dir",
        help = "Follow each command line symbolic link that points to a directory"
    )]
    dereference_argument_dir: bool,
    #[arg(
        long = "hide",
        help = "Do not list entries which matches PATTERN, overriden by -a or -A",
        value_name("PATTERN")
    )]
    hide: Option<String>,
    #[arg(long = "hyperlink", help = "Hyperlink file names WHEN")]
    hyperlink_when: Option<When>,
    #[arg(
        long = "indicator-style",
        help = "Append indicator with style WORD to entry names: none (default), slash (-p), file-type (--file-type), classify (-F)",
        default_value("none")
    )]
    indicator_style: Option<IndicatorWord>,
    #[arg(
        short = 'i',
        long = "inode",
        help = "Print the index number of each file"
    )]
    inode: bool,
    #[arg(
        short = 'I',
        long = "ignore",
        help = "Do not list entries which matches PATTERN",
        value_name("PATTERN")
    )]
    ignore_pattern: Option<String>,
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
    // TODO
    #[arg(
        short = 'N',
        long = "literal",
        help = "Print entry names without quoting"
    )]
    literal: bool,
    // TODO
    #[arg(short = 'o', help = "Like -l but do not list group information")]
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
    // TODO
    #[arg(
        long = "time",
        help = "Select which timestamp used to display or sort; access time (-u): atime, access, use; metadata change time (-c): ctime, status;  modified  time  (default): mtime, modification; birth time: birth, creation;\nWith -l, WORD determines which time to show; with --sort=time, sort by WORD (newest first)"
    )]
    time_display_sort: Option<TimeWord>,
    // TODO
    #[arg(
        long = "time-style",
        help = "Time/Date format of -l; TIME_STYLE syntax: {TODO}",
        value_name("TIME_STYLE")
    )]
    time_style: Option<String>,
    // TODO
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
    // TODO
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
    // TODO
    #[arg(short = 'x', help = "List entries by lines instead of columns")]
    list_columns: bool,
    // TODO
    #[arg(short = 'X', help = "Sort alphabetically by entry extension")]
    sort_extension: bool,
    // TODO
    #[arg(long = "zero", help = "End each output line with NUL, not newline")]
    end_nul: bool,
    // TODO
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
enum BlockSize {
    /// KiB (1024)
    K,
    /// MiB (1024^2)
    M,
    /// GiB (1024^3)
    G,
    /// TiB (1024^4)
    T,
    /// PiB (1024^5)
    P,
    /// EiB (1024^6)
    E,
    /// ZiB (1024^7)
    Z,
    /// YiB (1024^8)
    Y,
    /// RiB (1024^9)
    R,
    /// QiB (1024^10)
    Q,
    /// KB (1000)
    KB,
    /// MB (1000^2)
    MB,
    /// GB (1000^3)
    GB,
    /// TB (1000^4)
    TB,
    /// PB (1000^5)
    PB,
    /// EB (1000^6)
    EB,
    /// ZB (1000^7)
    ZB,
    /// YB (1000^8)
    YB,
    /// RB (1000^9)
    RB,
    /// QB (1000^10)
    QB,
}

#[derive(clap::ValueEnum, Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
enum When {
    #[default]
    Always,
    Auto,
    Never,
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
    AccessTime,
    MetadataChangeTime,
    ModifiedTime,
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
    for file in &cli.files {
        ls(&cli, file);
    }
}

fn ls(cli: &Cli, p: &PathBuf) {
    let mut dir = WalkDir::new(p).min_depth(1);
    if !cli.recursive {
        dir = dir.max_depth(1)
    }

    // First we get and collect all the entries into a vector of strings
    let entries: Vec<(String, PathBuf)> = dir
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
                e.into_path(),
            )
        })
        .collect();
    // Get longest entry
    let longest_entry = entries
        .clone()
        .iter()
        .map(|x| x.0.clone().chars().count())
        .max()
        .unwrap_or(0);
    // Create the lines variable we will use later
    let lines = treat_entries(cli, entries);

    // Finally trigger the right function
    if !cli.list {
        if !cli.recursive {
            normal_list(cli, lines, longest_entry);
        }
    } else {
        if !cli.recursive {
            list_list(cli, lines);
        }
    }
}

fn treat_entries(cli: &Cli, entries_list: Vec<(String, PathBuf)>) -> Vec<Vec<(String, PathBuf)>> {
    let term_size = termsize::get();
    let mut entries = entries_list;

    // Here we start treating the vector and variables
    // Sorting
    if !cli.no_sort {
        entries.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));
    }
    // If the all or almost all mode isn't activated we need to do some filtering
    if !cli.all || !cli.almost_all {
        entries = entries
            .into_iter()
            .filter_map(|f| if !f.0.starts_with(".") { Some(f) } else { None })
            .collect();
    } else if cli.all {
        entries.insert(0, (String::from("."), PathBuf::from("./")));
        entries.insert(1, (String::from(".."), PathBuf::from("../")));
    }

    // We start splitting up here
    // If no terminal size we can assume it was called either as a background process or some other
    // non-graphical process
    if term_size.is_none() {
        entries.iter().for_each(|entry| print!("{}", entry.0));
        print!("\n");
        exit(0);
    }

    let longest_entry = entries
        .iter()
        .map(|x| x.0.clone().chars().count())
        .max()
        .unwrap_or(0);

    // Get the maximum entries per line and use this to create a new Vec<Vec<String>>
    let entry_per_line = term_size.unwrap().cols as usize / (longest_entry + 2);
    entries
        .chunks(entry_per_line)
        .map(|s| s.into())
        .collect::<Vec<Vec<(String, PathBuf)>>>()
}

fn normal_list(cli: &Cli, lines: Vec<Vec<(String, PathBuf)>>, longest_entry: usize) {
    if lines.len() > 1 {
        for line in lines {
            for entry in line {
                let style = match entry.1.as_path().ptype() {
                    PathType::Directory => Style::new().bold().fg(Colour::Blue),
                    PathType::Executable => Style::new().bold().fg(Colour::Green),
                    PathType::Symlink => Style::new().bold().fg(Colour::Cyan),
                    _ => Style::new(),
                };
                let entry_format_string = format!(
                    "{: <width$}",
                    style.paint(entry.0.clone()).to_string()
                        + match entry.1.as_path().ptype() {
                            PathType::Symlink => "@",
                            PathType::Directory => "/",
                            PathType::Executable => "*",
                            _ => "",
                        },
                    width = longest_entry + 2
                );
                print!("{}", entry_format_string);
            }
            print!("\n");
        }
    } else {
        for line in lines {
            for entry in line {
                let style = match entry.1.as_path().ptype() {
                    PathType::Directory => Style::new().bold().fg(Colour::Blue),
                    PathType::Executable => Style::new().bold().fg(Colour::Green),
                    PathType::Symlink => Style::new().bold().fg(Colour::Cyan),
                    _ => Style::new(),
                };
                print!(
                    "{}{}  ",
                    style.paint(&entry.0),
                    match entry.1.as_path().ptype() {
                        PathType::Symlink => "@",
                        PathType::Directory => "/",
                        PathType::Executable => "*",
                        _ => "",
                    }
                );
            }
            print!("\n");
        }
    }
}

fn list_list(cli: &Cli, lines: Vec<Vec<(String, PathBuf)>>) {
    let mut entries: Vec<(
        String,
        usize,
        String,
        String,
        usize,
        usize,
        String,
        String,
        String,
    )> = vec![];
    for line in lines {
        for entry in line {
            let style = match entry.1.as_path().ptype() {
                PathType::Directory => Style::new().bold().fg(Colour::Blue),
                PathType::Executable => Style::new().bold().fg(Colour::Green),
                PathType::Symlink => Style::new().bold().fg(Colour::Cyan),
                _ => Style::new(),
            };

            let metadata_entry = entry.1.metadata().unwrap();

            // Get the permission string (Example: -rw-r--r--, octal 644)
            let perms_str = metadata_entry.permissions().mode_struct().to_string();
            // Get entries in directory, or 1 if its a file
            let dir_entries = if entry.1.is_dir() {
                WalkDir::new(&entry.1).max_depth(1).into_iter().count()
            } else {
                1
            };
            // Get owner and group
            let owner = users::get_current_username().unwrap_or(OsString::from("unknown"));

            let group = users::get_current_groupname().unwrap_or(OsString::from("unknown"));

            // Create timestamps
            let timestamp = 

            // Finally create the format string
            let entry_item = (
                perms_str,
                dir_entries,
                owner.to_str().unwrap().to_string(),
                group.to_str().unwrap().to_string(),
                metadata_entry.size() as usize,
                19 as usize,
                String::from("aug"),
                String::from("11.00"),
                style.paint(entry.0.clone()).to_string()
                    + match entry.1.as_path().ptype() {
                        PathType::Symlink => "@",
                        PathType::Directory => "/",
                        PathType::Executable => "*",
                        _ => "",
                    },
            );
            entries.push(entry_item);
        }
    }

    // All "longest-variables"
    let longest = (
        entries
            .clone()
            .iter()
            .map(|x| x.1.to_string().chars().count())
            .max()
            .unwrap_or(0),
        entries
            .clone()
            .iter()
            .map(|x| x.2.chars().count())
            .max()
            .unwrap_or(0),
        entries
            .clone()
            .iter()
            .map(|x| x.3.chars().count())
            .max()
            .unwrap_or(0),
        entries
            .clone()
            .iter()
            .map(|x| x.4.to_string().chars().count())
            .max()
            .unwrap_or(0),
        entries
            .clone()
            .iter()
            .map(|x| x.5.to_string().chars().count())
            .max()
            .unwrap_or(0),
    );

    println!("total {}", entries.len());
    entries.iter().for_each(|f| {
        println!(
            "{} {: >longest_dir$} {: >longest_user$} {: >longest_group$} {: >longest_size$} {: >longest_date$} {} {} {}",
            f.0,
            f.1,
            f.2,
            f.3,
            f.4,
            f.5,
            f.6,
            f.7,
            f.8,
            longest_dir = longest.0,
            longest_user = longest.1,
            longest_group = longest.2,
            longest_size = longest.3,
            longest_date = longest.4
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
    fn new(&self, p: PathBuf) -> FileTimestamps {
        let metadata = p.metadata().unwrap();
        let access = Timestamp {
            unix: metadata.atime(),
            datetime: DateTime::from_timestamp(metadata.atime(), 0).unwrap_or(DateTime::from_timestamp(0,0).unwrap())
        };
        let modified = Timestamp {
            unix: metadata.mtime(),
            datetime: DateTime::from_timestamp(metadata.mtime(), 0).unwrap_or(DateTime::from_timestamp(0,0).unwrap())
        };
        let metadata_change = Timestamp {
            unix: metadata.ctime(),
            datetime: DateTime::from_timestamp(metadata.ctime(), 0).unwrap_or(DateTime::from_timestamp(0,0).unwrap())
        };

        FileTimestamps {
            access,
            modified,
            metadata_change
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Timestamp {
    unix: i64,
    datetime: DateTime<Utc>
}
