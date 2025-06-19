use std::{env::args, fs::read_dir, io::ErrorKind, path::PathBuf};

// Required for FS size on Unix
use std::os::unix::fs::MetadataExt;

use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(
    version,
    about = "Summarize device usage of the set of FILEs, recursively for directories.",
    author = "Alexander Hübner"
)]
struct Cli {
    #[clap(value_parser, num_args = 1.., value_delimiter = ' ', required = true)]
    file: Vec<PathBuf>,

    #[arg(
        short = 'a',
        long = "all",
        help = "write counts for all files, not just directories"
    )]
    all: bool,
    #[arg(
        long = "apparent-size",
        help = "print apparent sizes rather than device usage; although
                          the apparent size is usually smaller, it may be
                          larger due to holes in ('sparse') files, internal
                          fragmentation, indirect blocks, and the like"
    )]
    apparent_size: bool,
    #[arg(short = 'c', long = "total", help = "produce a grand total")]
    total: bool,
    #[arg(
        short = 'L',
        long = "dereference",
        help = "dereference all symbolic links"
    )]
    dereference: bool,
    #[arg(
        short = 'P',
        long = "no-dereference",
        help = "don't follow any symbolic links (this is the default)"
    )]
    no_dereference: bool, // this is unused
    #[arg(
        short = 's',
        long = "summarize",
        help = "display only a total for each argument"
    )]
    summarize: bool,
    // TODO: missing options:
    // -0, --null            end each output line with NUL, not newline
    // -B, --block-size=SIZE  scale sizes by SIZE before printing them; e.g., ...
    // -b, --bytes           equivalent to '--apparent-size --block-size=1'
    // -D, --dereference-args  dereference only symlinks that are listed on the ...
    // -d, --max-depth=N     print the total for a directory (or file, with --all)
    //     --files0-from=F   summarize device usage of the NUL-terminated file names ...
    // -H                    equivalent to --dereference-args (-D)
    // -h, --human-readable  print sizes in human readable format (e.g., 1K 234M 2G)
    //     --inodes          list inode usage information instead of block usage
    // -k                    like --block-size=1K
    // -l, --count-links     count sizes many times if hard linked
    // -m                    like --block-size=1M
    // -S, --separate-dirs   for directories do not include size of subdirectories
    //     --si              like -h, but use powers of 1000 not 1024
    // -t, --threshold=SIZE  exclude entries smaller than SIZE if positive, ...
    //     --time            show time of the last modification of any file in the ...
    //     --time=WORD       show time as WORD instead of modification time:
    //     --time-style=STYLE  show times using STYLE, which can be: ...
    // -X, --exclude-from=FILE  exclude files that match any pattern in FILE
    //     --exclude=PATTERN    exclude files that match PATTERN
    // -x, --one-file-system    skip directories on different file systems
    //
    //
    // also environment variables can be found at the end of the coreutils du help page:
    // Display values are in units of the first available SIZE from --block-size,
    // and the DU_BLOCK_SIZE, BLOCK_SIZE and BLOCKSIZE environment variables.
    // Otherwise, units default to 1024 bytes (or 512 if POSIXLY_CORRECT is set).
    //
    // The SIZE argument is an integer and optional unit (example: 10K is 10*1024).
    // Units are K,M,G,T,P,E,Z,Y,R,Q (powers of 1024) or KB,MB,... (powers of 1000).
    // Binary prefixes can be used, too: KiB=K, MiB=M, and so on.
}

pub fn main() {
    let cli: Cli;
    // skip first arg if it happens to be "blutils"
    if args().next().unwrap().ends_with("/blutils") {
        cli = Cli::parse_from(args().skip(1))
    } else {
        cli = Cli::parse()
    };

    // NOTE: this is not GNU coreutils compliant
    if cli.dereference && cli.no_dereference {
        eprintln!(
            "du: invalid usage: --dereference (-L) and --no-dereference (-P) were both provided"
        );
        return;
    }

    let mut total = 0;
    for file in cli.file.iter() {
        total += du_print(file, &cli).unwrap_or(0);
    }

    if cli.total {
        print_bytes(total, "total");
    }
}

fn path_string(p: &PathBuf) -> String {
    p.to_string_lossy().to_string()
}

/// Converts bytes to KiB
fn format_bytes(bytes: u64 /*, format: */) -> u64 {
    // TODO: universal support for byte conversion to KiB MiB GiB, etc.
    bytes / 1024 + if bytes % 1024 != 0 { 1 } else { 0 }
}

fn print_bytes(bytes: u64, message: &str) {
    println!("{}\t{}", format_bytes(bytes), message);
}

fn du_print(file: &PathBuf, cli: &Cli) -> Option<u64> {
    let size = du(file, cli);
    if let Some(size) = size {
        print_bytes(size, &path_string(file));
    }
    return size;
}

fn du(file: &PathBuf, cli: &Cli) -> Option<u64> {
    let file_symlink = file.is_symlink();
    if !file.exists() && (cli.dereference || !file_symlink) {
        eprintln!(
            "du: cannot access '{}': No such file or directory",
            path_string(file)
        );
        return None;
    }

    let file_metadata = match if cli.dereference {
        file.metadata()
    } else {
        file.symlink_metadata()
    } {
        Ok(metadata) => metadata,
        Err(err) => {
            // NOTE: this is not GNU coreutils compliant
            eprintln!("du: cannot read metadata '{}': {}", path_string(file), err);
            return None;
        }
    };

    let mut file_size = if cli.apparent_size {
        file_metadata.len()
    } else {
        // linux st_blocks, which returns the st_blocks result in 512-byte units:
        // https://doc.rust-lang.org/std/os/linux/fs/trait.MetadataExt.html#tymethod.st_blocks
        file_metadata.blocks() * 512
    };

    if file.is_dir() && (cli.dereference || !file_symlink) {
        let dir_content = match read_dir(&file) {
            Ok(content) => content,
            Err(err) => {
                let msg = match err.kind() {
                    ErrorKind::PermissionDenied => "Permission denied",
                    _ => &err.to_string(),
                };
                eprintln!("du: cannot read directory '{}': {}", path_string(file), msg);
                return Some(file_size);
            }
        };
        for entry in dir_content {
            if let Err(err) = entry {
                // NOTE: this is not GNU coreutils compliant
                eprintln!(
                    "du: cannot access entry in directory '{}': {}",
                    path_string(file),
                    err
                );
                continue;
            }
            let entry = &entry.unwrap().path(); // NOTE: entry.metadata() is lost here
            file_size += if !cli.summarize && (cli.all || entry.is_dir()) {
                du_print(entry, cli).unwrap_or(0)
            } else {
                du(entry, cli).unwrap_or(0)
            };
        }
    }

    Some(file_size)
}
