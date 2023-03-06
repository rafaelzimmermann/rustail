mod argparse;
mod file_reader;

use std::fs::File;
use argparse::parser;
use file_reader::buffered_reader;
use crate::file_reader::buffered_reader::FileReader;

fn exit(result: Result<String, String>) {
    match result {
        Ok(v) => {
            println!("{}", v);
            std::process::exit(0)
        },
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        },
    }
}

fn warning(message: String) {
    eprintln!("Warning: {}", message);
}

fn main() {
    let files: Vec<String> = parser::file_paths();

    if files.is_empty() {
        match parser::app_name() {
            Some(p) => exit(Err::<String, String>(format!("Usage: {} [FILE]...", p))),
            None => exit(Err::<String, String>("Usage: rusttail [FILE]...".to_string())),
        }
    }

    for file_name in files {
        let mut reader: FileReader = buffered_reader::create(&file_name);
        reader.next();
        println!("{}", file_name);
    }
}
