use std::env;
use std::fs::create_dir;
use std::fs::read_dir;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use chrono::Utc;

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
    let metadata = vec![
        (
            Utc::now().format("%F %X %Z").to_string(),
            PathBuf::from("src/metadata/build"),
        ),
        (
            dir_contents.join(","),
            PathBuf::from("src/metadata/modules"),
        ),
        (
            env!("CARGO_PKG_VERSION").to_string()
                + if std::env::var("PROFILE").is_ok_and(|x| x == "debug") {
                    "-debug"
                } else {
                    ""
                },
            PathBuf::from("src/metadata/version"),
        ),
    ];

    // in case the dir doesnt exist we got to create it!
    _ = create_dir("src/metadata");
    for (data, path) in metadata {
        let mut file = match File::create(path) {
            Err(e) => panic!("Couldn't create build file: {}", e),
            Ok(file) => file,
        };
        match file.write_all(data.as_bytes()) {
            Err(e) => panic!("Error while writing to file: {}", e),
            _ => (),
        };
    }
}
