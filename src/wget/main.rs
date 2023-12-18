use std::io::{Read, Write};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let program = &args[0];

    if args.len() != 3 {
        eprintln!("Usage: {program} <host:port> <path>");
        return;
    }

    match std::net::TcpStream::connect(&args[1]) {
        Ok(mut stream) => {
            let request = format!("GET {} HTTP/1.1\r\nConnection: close\r\n\r\n", &args[2]);
            if let Err(err) = stream.write_all(request.as_bytes()) {
                eprintln!("Could not write to {}: {}", &args[1], err);
                return;
            }

            let mut buffer = Vec::new();
            if let Err(err) = stream.read_to_end(&mut buffer) {
                eprintln!("Could not read from {}: {}", &args[1], err);
                return;
            }

            match std::str::from_utf8(&buffer) {
                Ok(response) => {
                    println!("{} responed with:\n{}", &args[1], response);
                },
                Err(err) => {
                    eprintln!("Failed to parse response: {}", err);
                    return;
                },
            }
        },
        Err(err) => {
            eprintln!("Could not connect to {}: {}", &args[1], err);
            return;
        },
    };
}
