const MODULES: &str = include_str!("metadata/modules");
const VERSION: &str = include_str!("metadata/version");
const BUILD: &str = include_str!("metadata/build");

use std::{env::args, process::exit};
// All modules
mod cat;
mod ls;
mod mkdir;
mod rmdir;
mod utils;
mod mv;
mod cp;
mod rm;
mod ln;
// End of modules

fn main() {
    let arguments: Vec<String> = args().collect();
    let mut command = arguments[0].split("/").last().unwrap();
    if arguments.len() < 2 && command == "blutils" {
        help();
    } else if arguments.len() >= 2 && command == "blutils" && (arguments[1] == "--list" || arguments[1] == "-l")  {
        list();
    } else if command == "blutils" {
        command = arguments[1].as_str();
    };
    match command {
        "ls" => ls::main(),
        "cat" => cat::main(),
        "mkdir" => mkdir::main(),
        "rmdir" => rmdir::main(),
        "mv" => mv::main(),
        "cp" => cp::main(),
        "rm" => rm::main(),
        _ => help(),
    }
}

fn list() {
    MODULES.split(",").for_each(|val| println!("{}", val));
    exit(1)
}

fn help() {
    let mut help_string = String::new();

    help_string.push_str(format!("blutils {} ({})\n", VERSION, BUILD).as_str());
    help_string
        .push_str(format!("\nUsage:\n\tblutils [module] [arguments]\n\tblutils --list\n").as_str());
    help_string.push_str(format!("\nCurrently defined modules:\n\t[").as_str());
    MODULES
        .split(",")
        .for_each(|val| help_string.push_str(format!("{},", val).as_str()));
    help_string.push_str("]\n");
    println!("{}", help_string);
    exit(1)
}
