use std::{default, env::args, path::PathBuf};

use clap::{Args, Parser};

const PROGRAM: &str = "ls";

#[derive(Parser, Debug, Clone)]
#[command(version, about = "List files in source!", author = "Alexander Hübner")]
struct Cli {
    #[clap(value_parser, required = true)]
    source: Vec<PathBuf>,

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
    #[arg(
        long = "author",
        help = "With -l, print the author of each file",
        requires("list")
    )]
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
        help = "With -l, scale sizes by SIZE when printing them; e.g., '--block-size=M'; see SIZE format below",
        requires("list")
    )]
    block_size: Option<BlockSize>,
    // Done
    #[arg(
        short = 'b',
        long = "ignore-backups",
        help = "Do not list entries ending with ~ or a specified suffix",
        value_name("suffix"),
        default_missing_value("~")
    )]
    ignore_backups: Option<String>,
    #[arg(
        short = 'c',
        help = "With -lt: sort by, and show, ctime (time of last change of file status information); with -l: show ctime and sort by name; otherwise: sort by ctime, newest first",
        requires("list")
    )]
    show_ctime: bool,
    #[arg(
        short = 'C',
        help = "List entries by columns",
        default_value("true")
    )]
    column: bool,
    #[arg(
        long = "color",
        help = "Color the output WHEN",
        default_value("never")
    )]
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
    #[arg(
        short = 'f',
        help = "Do not sort, enable -aU, disable -ls --color"
    )]
    no_sort: bool,
    #[arg(
        short = 'F',
        long = "classify",
        help = "Append indicator (one of */=>@|) to entries WHEN"
    )]
    classify: Option<When>,
    #[arg(
        long = "file-type",
        help = "Likewise, except do not append '*'"
    )]
    file_type: bool,
    #[arg(
        long = "format",
        help = "Across -x, commas -m, horizontal -x, long -l, single-column -1, verbose -l, vertical -C"
    )]
    format: Option<FormatWord>,
    #[arg(
        long = "full-time",
        help = "Like -l  --time-style=full-iso"
    )]
    alias_list_time_full_iso: bool,
    #[arg(
        short = 'g',
        help = "Like -l but does not list owner"
    )]
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
    #[arg(
        long = "hyperlink",
        help = "Hyperlink file names WHEN"
    )]
    file_type: Option<When>,
    #[arg(
        long = "indicator-style",
        help = "Append indicator with style WORD to entry names: none (default), slash (-p), file-type (--file-type), classify (-F)"
    )]
    indicator_style: Option<IndicatorWord>,
    // Done
    #[arg(
        short = 'l',
        long = "link",
        help = "Hard link files instead of copying"
    )]
    link: bool,
    //Done
    #[arg(
        short = 'L',
        long = "dereference",
        help = "Always follow symbolic links in SOURCE",
        conflicts_with("no_dereference")
    )]
    dereference: bool,
    //Done
    #[arg(
        short = 'P',
        long = "no-dereference",
        help = "Never follow symbolic links in SOURCE",
        conflicts_with("dereference")
    )]
    no_dereference: bool,
    //Done
    #[arg(short = 'p', help = "Same as --preserve=mode,ownership,timestamps")]
    alias_mode_own_time: bool,
    // Done
    #[arg(long = "preserve", help = "Preserve the specified attributes")]
    preserve: Option<Vec<Attributes>>,
    // Done
    #[arg(long = "no-preserve", help = "Don't preserve the specified attributes")]
    no_preserve: Option<Vec<Attributes>>,
    //Done
    #[arg(long = "parents", help = "Use full source file name under DIRECTORY")]
    parents: bool,
    // Done
    #[arg(
        short = 'R',
        long = "recursive",
        help = "Copy directories recursively",
        short_alias('r')
    )]
    recursive: bool,
    // Done
    #[arg(
        long = "remove-destination",
        help = "Remove each existing destination file before attempting to open it (contrast with --force)"
    )]
    remove_destination: bool,
    // Done
    #[arg(
        long = "strip-trailing-slashes",
        help = "Remove any trailing slashes from each SOURCE argument"
    )]
    strip_trailing_slashes: bool,
    // Done
    #[arg(
        short = 's',
        long = "symbolic-link",
        help = "Make symbolic links instead of copying"
    )]
    symbolic_link: bool,
    // Done
    #[arg(
        short = 'S',
        long = "suffix",
        help = "Specify a backup suffix (Text appended to the end of a backup filename)"
    )]
    suffix: Option<String>,

    // Done
    #[arg(
        short = 't',
        long = "target-directory",
        help = "Treat destination as a directory"
    )]
    target_directory: bool,

    // Done
    #[arg(
        short = 'T',
        long = "no-target-directory",
        help = "Treat destination as a normal file"
    )]
    no_target_directory: bool,
    // Planned for later updates
    //#[arg(long = "update", help = "Control which existing files are updated")]
    //update: Option<Update>,
    #[arg(short = 'v', long = "verbose", help = "explain whats being done")]
    verbose: bool,
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
    Never
}

#[derive(clap::ValueEnum, Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum FormatWord {
    Across, 
    Commas, 
    Horizontal,
    Long,
    SingleColumn,
    Verbose,
    Vertical
}

#[derive(clap::ValueEnum, Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum IndicatorWord {
    None,
    Slash,
    FileType,
    Classify
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
}
