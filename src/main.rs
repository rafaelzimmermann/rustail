mod argparse;
mod file_reader;

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

fn print_buffer(b: &[u8]) {
    let s = match std::str::from_utf8(b) {
        Ok(v) => println!("{}", v),
        Err(e) => return,
    };
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
        let mut reader =  match buffered_reader::create(&file_name) {
            Ok(reader) => reader,
            Err(error) => {
                exit(Err::<String, String>(error.message.to_string()));
                return;
            },
        };
        println!("{}\n", file_name);
        loop {
            let result = reader.next();
            match result {
                Ok(r) => print_buffer(r),
                Err(_) => exit(Ok("".to_string())), // TODO: Handle error
            }
        }
    }
}
