fn host_game(port: u16) {

}

fn join_game(host: &str) {

}

fn main() {
    let mut args = std::env::args();
    let program = args.next().unwrap();

    match args.next().as_deref() {
        Some("host") => match args.next() {
            Some(port) => match port.parse::<u16>() {
                Ok(port) => host_game(port),
                Err(_) => eprintln!("Invalid port number: {port}")
            },
            None => eprintln!("No port number provided")
        },
        Some("join") => match args.next() {
            Some(host) => join_game(&host),
            None => eprintln!("No host provided")
        },
        _ => {
            eprintln!("Usage: {program} host <port>");
            eprintln!("       {program} join <host:port>");
            return;
        }
    }
}
