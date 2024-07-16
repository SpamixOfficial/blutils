pub fn log(verbose: bool, message: String) {
    if verbose {
        println!("[log] {}", message)
    }
}
