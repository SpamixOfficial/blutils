use std::{
    any::Any,
    fmt::Display,
    io::{Error, Read, Result},
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

// Stolen from https://stackoverflow.com/a/42773525
pub fn libc_wrap<T: Ord + Default>(num: T) -> Result<T> {
    if num < T::default() {
        return Err(Error::last_os_error());
    }
    Ok(num)
}

pub fn wrap<T: Any, M: Display>(result: Result<T>, prog: M) -> T {
    let val = match result {
        Ok(val) => val,
        Err(e) => {
            let mut error_code = 1;
            if let Some(os_error) = e.raw_os_error() {
                eprintln!("{}: Error: {}", prog, e.to_string());
                error_code = os_error;
            } else {
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
    let default_option: char = match d {
        true => 'y',
        false => 'n',
    };
    println!("{} {}", prompt_options, question);
    let mut input = [0];
    let _ = std::io::stdin().read(&mut input);
    if input[0].to_ascii_lowercase() as char == default_option {
        return true;
    } else {
        return false;
    };
}
