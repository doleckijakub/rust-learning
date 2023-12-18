use std::{env, fs};

fn main() {
    let mut args = env::args();
    let program = args.next().unwrap();

    if args.len() == 0 {
        eprintln!("Usage: {program} <file1> <file2> ...");
        return;
    }

    for file in args {
        match fs::read_to_string(&file) {
            Ok(content) => println!("{content}"),
            Err(err) => eprintln!("{program}: {file}: {err}")
        }
    }
}
