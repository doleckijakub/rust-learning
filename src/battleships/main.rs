enum GameMessage {
    BoardReady,

    Shoot(u8, u8),

    Hit(u8, u8),
    Miss(u8, u8),
    Sunk(u8, u8),
}

#[derive(Copy, Clone)]
enum BoardState {
    Unknown,
    Ship,
    Miss,
    Hit,
}

const BOARD_SIZE: usize = 10;

struct Board {
    state: [[BoardState; BOARD_SIZE]; BOARD_SIZE]
}

impl Board {
    fn new() -> Self {
        Self { state: [[BoardState::Unknown; BOARD_SIZE]; BOARD_SIZE] }
    }
}

fn setup_board() -> Board {
    let ships: Vec<u8> = vec![ 5, 4, 3, 3, 2 ];
    let mut ships: Vec<((i8, i8, u8), u8)> = ships.iter().map(|&l| ((-1, -1, 0), l)).collect();
    let mut ship_index = 0;

    fn is_ship(ships: &[((i8, i8, u8), u8)], x: i8, y: i8) -> bool {
        for ship in ships {
            if ship.0.0 != -1 {
                let (dx, dy) = match ship.0.2 % 4 {
                    0 => (1, 0),
                    1 => (0, 1),
                    2 => (-1, 0),
                    3 => (0, -1),
                    _ => (0, 0)
                };

                for i in 0..ship.1 {
                    if ship.0.0 + dx == x && ship.0.1 + dy == y {
                        return true
                    }
                }
            }
        }

        false
    } 

    let mut done = false;
    let border = "##".repeat(BOARD_SIZE + 2);

    print!("\x1B[2J");
    
    while !done {
        println!("\x1B[H{} Ships:", border);
        for y in 0..BOARD_SIZE {
            print!("##");
            for x in 0..BOARD_SIZE {
                if is_ship(&ships, x as i8, y as i8) {
                    print!("&&");
                } else {
                    print!("  ");
                };
            }

            print!("##");

            if 1 <= y && y <= ships.len() {
                if y - 1 == ship_index {
                    print!(" >");
                }
                    
                let ship = ships.iter().nth(y - 1).unwrap();
                print!(" {}", (if ship.0.0 == -1 { "%%" } else { "**" }).repeat(ship.1.into()));
            }

            println!("");
        }
        println!("{}", border);
    }

    let mut board = Board::new();

    

    board
}

fn host_game(port: u16) -> std::io::Result<()> {
    // let addr = format!("0.0.0.0:{port}");
    
    // let listener = std::net::TcpListener::bind(addr)?;
    
    // let (socket, _) = listener.accept()?;

    // setup board & wait for client's board

    // let mut buffer = [0 as u8; 64];

    let mut my_board = setup_board();

    // game loop
    
    Ok(())
}

fn join_game(host: &str) -> std::io::Result<()> {
    // let mut stream = std::net::TcpStream::connect(host)?;
    // setup board & wait for server's board
    // game loop

    Ok(())
}

fn main() {
    let mut args = std::env::args();
    let program = args.next().unwrap();

    match args.next().as_deref() {
        Some("host") => match args.next() {
            Some(port) => match port.parse::<u16>() {
                Ok(port) => if let Err(err) = host_game(port) {
                    eprintln!("Error: {err}");
                    return;
                },
                Err(_) => eprintln!("Invalid port number: {port}")
            },
            None => eprintln!("No port number provided")
        },
        Some("join") => match args.next() {
            Some(host) => if let Err(err) = join_game(&host) {
                eprintln!("Error: {err}");
                return;
            },
            None => eprintln!("No host provided")
        },
        _ => {
            eprintln!("Usage: {program} host <port>");
            eprintln!("       {program} join <host:port>");
            return;
        }
    }
}
