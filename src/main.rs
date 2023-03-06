mod argparse;

use argparse::parser;

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

fn main() {
    let files: Vec<String> = parser::files();

    if files.is_empty() {
        match parser::app_name() {
            Some(p) => exit(Err::<String, String>(format!("Usage: {} [FILE]...", p))),
            None => exit(Err::<String, String>("Usage: rusttail [FILE]...".to_string())),
        }
    }

    for x in files {
        println!("{}", x);
    }
}
