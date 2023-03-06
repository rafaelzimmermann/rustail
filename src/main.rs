use std::env;
use std::path::Path;
use std::ffi::OsStr;

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

fn arg_files() -> Vec<String> {
    return env::args().skip(1).collect();
}

fn app_name() -> Option<String> {
    return env::args().next()
        .as_ref()
        .map(Path::new)
        .and_then(Path::file_name)
        .and_then(OsStr::to_str)
        .map(String::from)
}

fn main() {
    let files: Vec<String> = arg_files();

    if files.is_empty() {
        match app_name() {
            Some(p) => exit(Err::<String, String>(format!("Usage: {} [FILE]...", p))),
            None => exit(Err::<String, String>("Usage: rusttail [FILE]...".to_string())),
        }
    }

    for x in files {
        println!("{}", x);
    }
}
