use std::io::{Result, Error};

pub fn log(verbose: bool, message: String) {
    if verbose {
        println!("[log] {}", message)
    }
}

// Stolen from https://stackoverflow.com/a/42773525
pub fn check_libc_err<T: Ord + Default>(num: T) -> Result<T> {
    if num < T::default() {
        return Err(Error::last_os_error());
    }
    Ok(num)
}
