use std::env;

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
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        match args.get(0) {
            Some(p) => exit(Err::<String, String>(format!("Usage: {} [FILE]...", p))),
            None => exit(Err::<String, String>("Usage: rusttail [FILE]...".to_string())),
        }
        
    }

    for x in args.iter().skip(1) {
        println!("{}", x);
    }
}
