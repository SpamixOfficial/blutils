use std::{default, env::args, path::{Path, PathBuf}, process::exit};

use clap::{Args, Parser};
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
    #[clap(value_parser, required = true, value_name("FILE"))]
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

    //Done
    #[arg(long = "author", help = "With -l, print the author of each file")]
    author: bool,
    //Done
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
    // Done
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
    // Done
    #[arg(short = 'l', help = "Use a long listing format")]
    list: bool,
    //Done
    #[arg(
        short = 'L',
        long = "dereference",
        help = "Use dereferenced symbolic link information in result instead of symbolic link itself"
    )]
    dereference: bool,
    #[arg(
        short = 'm',
        help = "Fill width with a comma separated list of entries"
    )]
    fill_comma: bool,
    #[arg(
        short = 'n',
        long = "numeric-uid-grid",
        help = "Like l, but list numeric user and group IDs"
    )]
    numeric_list: bool,
    #[arg(
        short = 'N',
        long = "literal",
        help = "Print entry names without quoting"
    )]
    literal: bool,
    #[arg(short = 'o', help = "Like -l but do not list group information")]
    no_group_list: bool,
    #[arg(short = 'p', help = "Append / to directories")]
    slash: bool,
    #[arg(
        short = 'q',
        long = "hide-control-chars",
        help = "Print ? instead of nongraphic characters"
    )]
    hide_control_chars: bool,
    #[arg(
        long = "show-control-chars",
        help = "Show nongraphic as-is (No special visualization)"
    )]
    show_control_chars: bool,
    #[arg(
        short = 'Q',
        long = "quote-name",
        help = "Enclose entry names in double quotes"
    )]
    quote_name: bool,
    #[arg(
        long = "quoting-style",
        help = "Use  quoting  style WORD for entry names: literal, locale, shell, shell-always, shell-escape, shell-escape-always, c, escape (overrides QUOTING_STYLE environment variable)"
    )]
    quoting_style: Option<QuotingWord>,
    #[arg(short = 'r', long = "reverse", help = "Reverse order while sorting")]
    reverse: bool,
    #[arg(
        short = 'R',
        long = "recursive",
        help = "List subdirectories recursively"
    )]
    recursive: bool,
    #[arg(
        short = 's',
        long = "size",
        help = "Print the allocated size of each file, in blocks"
    )]
    size_blocks: bool,
    #[arg(short = 'S', help = "Sort by file size, largest first")]
    size_sort: bool,
    #[arg(
        long = "sort",
        help = "Sort by WORD instead of name: none (-U), size (-S), time (-t), version (-V), extension (-X), width"
    )]
    sort_word: Option<SortWord>,
    #[arg(
        long = "time",
        help = "Select which timestamp used to display or sort; access time (-u): atime, access, use; metadata change time (-c): ctime, status;  modified  time  (default): mtime, modification; birth time: birth, creation;\nWith -l, WORD determines which time to show; with --sort=time, sort by WORD (newest first)"
    )]
    time_display_sort: Option<TimeWord>,
    #[arg(
        long = "time-style",
        help = "Time/Date format of -l; TIME_STYLE syntax: {TODO}",
        value_name("TIME_STYLE")
    )]
    time_style: Option<String>,
    #[arg(short = 't', help = "Sort by time")]
    time_sort: bool,
    #[arg(
        short = 'T',
        long = "tabsize",
        help = "Assume tabs stop at each COLS instead of 8",
        value_name("COLS")
    )]
    tab_size: Option<u32>,
    #[arg(
        short = 'u',
        help = "With -lt: sort by, and show, access time; with -l: show access time and sort by name; otherwise: sort by access time, newest first"
    )]
    sort_access_time: bool,
    #[arg(short = 'U', help = "Do not sort; list entries in directory order")]
    no_sort: bool,
    #[arg(short = 'v', help = "Natural sort of (version) numbers within text")]
    sort_version: bool,
    #[arg(
        short = 'w',
        long = "width",
        help = "Set output width to COLS, 0 means no limit",
        value_name("COLS")
    )]
    output_width: Option<u32>,
    #[arg(short = 'x', help = "List entries by lines instead of columns")]
    list_columns: bool,
    #[arg(short = 'X', help = "Sort alphabetically by entry extension")]
    sort_extension: bool,
    #[arg(long = "zero", help = "End each output line with NUL, not newline")]
    end_nul: bool,
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
    for file in &cli.files {
        ls(&cli, file);
    }
}

fn ls(cli: &Cli, p: &PathBuf) {
    let mut dir = WalkDir::new(p); 
    dbg!(&p); 
    if !cli.recursive {
        dir = dir.max_depth(1)
    }
    let term_size = termsize::get();
    let entries: Vec<String> = dir.into_iter().filter_map(|e| e.ok()).map(|e| e.into_path().file_name().unwrap().to_str().unwrap().to_string()).collect();
    
    // If no terminal size we can assume it was called either as a background process or some other
    // non-graphical process
    if term_size.is_none() {
        entries.iter().for_each(|entry| print!("{entry}"));
        print!("\n");
        exit(0);
    }

    let longest_entry = entries.iter().map(|x| x.clone().chars().count()).max().unwrap_or(0);
    let entry_per_line = (longest_entry+2)/term_size.unwrap().cols as usize;
    for line in entries.chunks(entry_per_line).collect() {

    };
}
