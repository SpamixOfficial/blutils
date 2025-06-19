use libc::{
    getgrgid, getpwuid, getuid, S_IFIFO, S_IFMT, S_IFSOCK, S_IRGRP, S_IROTH, S_IRUSR, S_ISVTX,
    S_IWGRP, S_IWOTH, S_IWUSR, S_IXGRP, S_IXOTH, S_IXUSR,
};
use std::{
    any::Any,
    ffi::{c_char, CStr},
    fmt::{Display, Pointer},
    fs::{Metadata, Permissions},
    io::{Error, Read, Result},
    os::{
        linux::fs::MetadataExt as lmext,
        unix::fs::{MetadataExt, PermissionsExt},
    },
    path::Path,
    process::exit,
};

pub fn log<T: Display>(verbose: bool, message: T) {
    if verbose {
        println!("[log] {}", message)
    }
}

pub fn debug<T: Display>(debug: bool, message: T) {
    if debug {
        println!("[debug] {}", message)
    }
}

pub fn is_sudo() -> bool {
    unsafe {
        if getuid() != 0 {
            return false;
        } else {
            return true;
        };
    };
}

pub fn charp_to_string(charp: *mut c_char) -> String {
    unsafe { String::from(CStr::from_ptr(charp).to_str().unwrap()) }
}

// Stolen from https://stackoverflow.com/a/42773525
pub fn libc_wrap<T: Ord + Default>(num: T) -> Result<T> {
    if num < T::default() {
        return Err(Error::last_os_error());
    }
    Ok(num)
}

pub fn c_escape(contents: String, show_tabs: bool) -> String {
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
        } else if ch == '\t' && !show_tabs {
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

pub trait PathExtras {
    fn type_display(&self) -> Box<dyn Display>;
    fn ptype(&self) -> PathType;
    fn str_classify(&self, no_exe: bool, when: i8) -> String;
}

pub fn test_mode(m: Metadata, t: u32) -> bool {
    return (m.permissions().mode() & S_IFMT) == t;
}

impl PathExtras for Path {
    fn type_display(&self) -> Box<dyn Display> {
        match self.ptype() {
            PathType::Directory => Box::new("directory"),
            PathType::Executable => Box::new("executable"),
            PathType::File => Box::new("file"),
            PathType::Symlink => Box::new("link"),
            PathType::FIFO => Box::new("fifo"),
            PathType::Door => Box::new("door"),
            PathType::Socket => Box::new("socket"),
        }
    }
    fn ptype(&self) -> PathType {
        if self.is_dir() {
            return PathType::Directory;
        }
        if self.is_symlink() {
            return PathType::Symlink;
        }
        if self.metadata().is_ok()
            && (self.metadata().unwrap().permissions().mode() & (S_IXUSR | S_IXGRP)) != 0
        {
            return PathType::Executable;
        }

        if self.metadata().is_ok() && test_mode(self.metadata().unwrap(), S_IFIFO) {
            return PathType::FIFO;
        }

        if self.metadata().is_ok() && test_mode(self.metadata().unwrap(), S_IFSOCK) {
            return PathType::Socket;
        }

        // GNU has this, so I included it. Only seems to be necessary on solaris >2.5
        /*
           2025/06/19 - Continuing this project today, will most likely start removing stuff I deem unnecessary to shrink the binary size + make it easier to maintain
                        This also means that this option will most likely be gone by version 0.1.2
        */
        if self.metadata().is_ok() && test_mode(self.metadata().unwrap(), 0) {
            return PathType::Door;
        }

        PathType::File
    }
    fn str_classify(&self, no_exe: bool, when: i8) -> String {
        let result = match self.ptype() {
            PathType::Symlink => "@",
            PathType::Directory => "/",
            PathType::Executable => {
                if !no_exe {
                    "*"
                } else {
                    " "
                }
            }
            PathType::Door => ">",
            PathType::FIFO => "|",
            PathType::Socket => "=",
            _ => " ",
        }
        .to_string();
        match when {
            0 => result,
            1 => {
                if termsize::get().is_some() {
                    result
                } else {
                    " ".to_string()
                }
            }
            _ => " ".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum PathType {
    File,
    Directory,
    Symlink,
    Executable,
    FIFO,
    Socket,
    Door,
}

pub trait MetadataPlus {
    fn owner(&self) -> String;
    fn group(&self) -> String;
}

impl MetadataPlus for Metadata {
    fn group(&self) -> String {
        let group: String;
        let gid = self.st_gid();

        unsafe {
            let group_pointer = getgrgid(gid);
            if group_pointer.is_null() {
                group = String::from("unknown");
                return group;
            };
            // pointer to string conversion (beautiful, I know)
            group = charp_to_string((*group_pointer).gr_name);
        };
        group
    }

    fn owner(&self) -> String {
        let owner: String;
        let uid = self.st_uid();

        unsafe {
            let pw_pointer = getpwuid(uid);
            if pw_pointer.is_null() {
                owner = String::from("unknown");
                return owner;
            }
            // fancy oneliner
            owner = charp_to_string((*pw_pointer).pw_name);
        };

        owner
    }
}

pub trait PermissionsPlus {
    fn mode_struct(&self) -> ModeWrapper;
}

impl PermissionsPlus for Permissions {
    fn mode_struct(&self) -> ModeWrapper {
        let mode_bits = &self.mode();
        let owner = Mode {
            read: (mode_bits & S_IRUSR) != 0,
            write: (mode_bits & S_IWUSR) != 0,
            execute: (mode_bits & S_IXUSR) != 0,
        };
        let group = Mode {
            read: (mode_bits & S_IRGRP) != 0,
            write: (mode_bits & S_IWGRP) != 0,
            execute: (mode_bits & S_IXGRP) != 0,
        };
        let others = Mode {
            read: (mode_bits & S_IROTH) != 0,
            write: (mode_bits & S_IWOTH) != 0,
            execute: (mode_bits & S_IXOTH) != 0,
        };
        ModeWrapper {
            sticky_bit: (mode_bits & S_ISVTX) != 0,
            owner,
            group,
            others,
        }
    }
}

impl ModeWrapper {
    pub fn to_string(&self) -> String {
        format!(
            "{}{}{}{}",
            if self.sticky_bit { "d" } else { "-" },
            self.owner.to_string(),
            self.group.to_string(),
            self.others.to_string()
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ModeWrapper {
    sticky_bit: bool,
    owner: Mode,
    group: Mode,
    others: Mode,
}

impl Mode {
    pub fn to_string(&self) -> String {
        format!(
            "{}{}{}",
            if self.read { "r" } else { "-" },
            if self.write { "w" } else { "-" },
            if self.execute { "x" } else { "-" }
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Mode {
    read: bool,
    write: bool,
    execute: bool,
}

pub fn wrap<T: Any, M: Display>(result: Result<T>, prog: M, silent: bool) -> T {
    let val = match result {
        Ok(val) => val,
        Err(e) => {
            let mut error_code = 1;
            if let Some(os_error) = e.raw_os_error() {
                if !silent {
                    eprintln!("{}: Error: {}", prog, e.to_string());
                }
                error_code = os_error;
            } else if !silent {
                eprintln!("{}: Error: {}", prog, e.to_string())
            };
            exit(error_code)
        }
    };
    return val;
}

pub fn prompt<T: Display>(question: T, d: bool) -> bool {
    let prompt_options = match d {
        true => "Y/n",
        false => "N/y",
    };
    println!("{} {}", prompt_options, question);
    let mut input = [0];
    let _ = std::io::stdin().read(&mut input);
    match input[0].to_ascii_lowercase() as char {
        'y' => true,
        'n' => false,
        _ => return d,
    }
}