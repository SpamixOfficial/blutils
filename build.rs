use std::env;
use std::fs::read_dir;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use chrono::{DateTime, Utc};

fn main() {
    let black_list: Vec<String> = vec![
        String::from("main"),
        String::from("utils"),
        String::from("metadata"),
    ];
    let mut dir_contents: Vec<String> = read_dir("src")
        .unwrap()
        .map(|f| {
            f.unwrap()
                .path()
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
        })
        .collect::<Vec<String>>();
    dir_contents.retain(|v| !black_list.contains(v));
    let metadata = vec![Utc::now().format("%F %X %Z").to_string(), dir_contents.join(","), env!("CARGO_PKG_VERSION").to_string()];

    let mut i = 0;
    for f in read_dir("src/metadata").unwrap() {
        let f = f.unwrap();
        let mut file = match File::create(f.path()) {
            Err(e) => panic!("Couldn't create build file: {}", e),
            Ok(file) => file,
        };
        match file.write_all(metadata[i].as_bytes()) {
            Err(e) => panic!("Error while writing to file: {}", e),
            _ => (),
        };
        i+=1;
    }
}
