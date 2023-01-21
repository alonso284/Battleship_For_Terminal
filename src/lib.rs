use std::net::TcpStream;
use std::net::TcpListener;
use std::net::SocketAddr;
use std::io::Read;
use std::io::Write;
use termion::color;
use termion::terminal_size;
use rand::random;
use std::io;

// pub const IP: &str = "10.174.89.9";
pub const IP: &str = "192.168.1.69";
pub const PORT: &str = "7878";
const N_SHIPS:usize = 8;

pub struct Player {
    pub socket: TcpStream,
    pub address: SocketAddr,
    board: [[Cell;8];8],
    ship_units_left: usize,
}
#[derive(Debug)]
pub struct Fleet {
    carrier_count: usize,
    battleship_count: usize,
    cruiser_count: usize,
    destroyer_count: usize,
    board: String,
    ships_locations: [Option<(Position, Ships)>; 10],
    ships_placed: usize,
}

#[derive(Debug, Copy, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
    pub orientation: Orientation,
}

#[derive(Debug, Copy, Clone)]
pub enum Orientation {
    Vertical,
    Horizontal,
}

#[derive(Debug, Copy, Clone)]
pub enum Ships {
    Carrier,
    Battleship,
    Cruiser,
    Destroyer,
}

#[derive(Copy, Clone)]
enum Cell {
    Ship,
    Free,
    Shot,
}

pub enum Shot {
    Hit,
    Miss,
}

impl Fleet {
    // new Fleet
    pub fn new() -> Fleet {
        Fleet {
            carrier_count: 1,
            battleship_count: 2,
            cruiser_count: 3,
            destroyer_count: 4,
            board: String::from(format!("{}", "O".repeat(64))),
            ships_locations: [None; 10],
            ships_placed: 0,
        }
    }
    pub fn add_ship(&mut self, ship_type: Ships, (x, y):(usize, usize), orientation: Orientation) -> Result<(), String> {

        let position = Position {
            x, y, orientation,
        };
        // Check if ships of that type are left
        if match ship_type {
            Ships::Carrier => self.carrier_count,
            Ships::Battleship => self.battleship_count,
            Ships::Cruiser => self.cruiser_count,
            Ships::Destroyer => self.destroyer_count,
        } <= 0 {
            return Err(String::from(format!("No {:?}s left", ship_type)));
        }

        // Get size of ship to free space
        let free_space = size_of(&ship_type);

        // Get if there is enough space in the board to place ship
        if match position.orientation {
            Orientation::Vertical => position.x,
            Orientation::Horizontal => position.y,
        } + free_space - 1 > 8 {
            return Err(String::from("There is no enough space to place ship"));
        }

        // Get splice of string and coordinates to access
        let mut board_bytes = [0u8;64];
        let mut iterator = self.board.bytes();
        for i in 0..64{
            board_bytes[i] = iterator.next().unwrap();
        }

        let (mut x, mut y) = (position.x - 1, position.y - 1);

        // Check if there are no ships in the way
        for _ in 0..free_space {
            let cell = x*8 + y;
            if let b'X' =  board_bytes[cell] {
                return Err(String::from("One of the cells has been already occupied"));
            }
            match position.orientation {
                Orientation::Vertical => x += 1,
                Orientation::Horizontal => y += 1,
            }
        }
        
        // Modify board and place the shape
        let (mut x, mut y) = (position.x - 1, position.y - 1);
        for _ in 0..free_space {
            let cell = x*8 + y;
            board_bytes[cell] = b'X';
            match position.orientation {
                Orientation::Vertical => x += 1,
                Orientation::Horizontal => y += 1,
            }
        }

        // Add location of ship
       self.ships_locations[self.ships_placed] = Some((position, ship_type));
       // Aument the count of ships placed
       self.ships_placed += 1;
       // Reduce count of ship type
        match ship_type {
            Ships::Carrier => self.carrier_count -= 1,
            Ships::Battleship => self.battleship_count -= 1,
            Ships::Cruiser => self.cruiser_count -= 1,
            Ships::Destroyer => self.destroyer_count -= 1,
        }
        self.board = String::from_utf8_lossy(&board_bytes).to_string();

        Ok(())
    }

    pub fn get_board(&self) -> &String {
        &self.board
    }

    pub fn print_status(&self){
        for ship in self.ships_locations{
            match ship {
            Some(ship) => println!("There is a {:?} in position ({}, {}) with orientation {:?}", ship.1, ship.0.x, ship.0.y, ship.0.orientation),
            None => {},
            }
        }
    }

    pub fn all_ships_placed(&self) -> bool {
        self.ships_placed == 10
    }
}


fn size_of(ship: &Ships) -> usize {
    match ship {
        Ships::Carrier => 5,
        Ships::Battleship => 4,
        Ships::Cruiser => 3,
        Ships::Destroyer => 2,
    }
}


// Player functions
impl Player {

    // Generate new player with their own board
    pub fn new(socket: TcpStream, address: SocketAddr) -> Player{
        let mut board = [[Cell::Free;8];8];
        let mut count = 0;

        // Generate random spaces with ships
        while count < N_SHIPS{
            let (x, y):(usize,usize) = (random(), random());
            let (x, y) = (x%8, y%8);
            if let Cell::Ship = board[x][y] {
                continue;
            }
            count += 1;
            board[x%8][y%8] = Cell::Ship;
        }

        Player{
            socket,
            address,
            board,
            ship_units_left: N_SHIPS,
        }
    }

    // Generate a String of the board to send to the players
    pub fn board_str(&self, show_ships: bool) -> String {

        let mut board = String::new();
        for iterator in self.board {
            for cell in iterator {
               board.push(match cell {
                   Cell::Shot => 'X',
                   Cell::Ship => { match show_ships {
                       true => 'S',
                       false => 'O',
                    }
                   },
                   Cell::Free => 'O',
                });
            }
        }
        board
    }

    // Change boards state to receive shot
    pub fn receive_shot(&mut self, turn:String) -> Shot {
       
        let (x, y) = Self::parse_turn(turn);
        let shot = Cell::clone(&self.board[x][y]);

        self.board[x][y] = Cell::Shot;

        match shot {
            Cell::Ship => {
                self.ship_units_left -= 1;
                Shot::Hit
            },
            Cell::Free => Shot::Miss,
            _ => Shot::Miss,
        }
    }

    // Return true if all ship units have been sunk
    pub fn is_sunk(&self) -> bool {
        self.ship_units_left <= 0
    }

    fn parse_turn(turn:String) -> (usize, usize){
        let mut it = turn.chars();
        ((it.next().unwrap() as u8 - 'A' as u8) as usize, (it.next().unwrap() as u8 - '1' as u8) as usize)
    }

}

pub fn get_play() -> Result<String, String> {
    let mut play = String::new();
    io::stdin().read_line(&mut play).unwrap();
    let play = play.trim();

    if play.len() != 2 {
            return Err(String::from("Invalid Turn, size of coordinates is too long"));
    }

    let mut iterator = play.chars();
    let x = iterator.next().unwrap();
    let y = iterator.next().unwrap();

    if x < 'A' || 'H' < x{
        return Err(String::from("First coordinate must be between A and H"));
    }
    if y < '1' || '8' < y{
        return Err(String::from("Second coordinate must be between 1 and 8"));
    }

    Ok(play.to_string())
}

pub fn send(stream:&mut TcpStream, msg: &[u8]) -> Result<(), String> {
    stream.write(msg).unwrap();
    stream.flush().unwrap();
    std::thread::sleep(std::time::Duration::new(2,0));
    // println!("Sending message: {}", String::from_utf8_lossy(msg).to_string());
    Ok(())
}

pub fn rcv(stream:&mut TcpStream) -> Result<String, String>{
    let mut buffer = [0u8;1024];
    stream.read(&mut buffer).unwrap();
    // println!("Recieving message: {}", String::from_utf8_lossy(&buffer));
    Ok(String::from_utf8_lossy(&buffer).to_string())
}

pub fn connect_player(listener: &TcpListener) -> Result<Player, std::io::Error> {
    let mut player = match listener.accept() {
        Ok((socket, address)) => Player::new(socket, address),
        Err(e) => return Err(e),
    };

    send(&mut player.socket, b"TRUE").unwrap();

    println!("Player connected in {:?}", player.address);

    Ok(player)
}

pub fn print_board(board: &String){
    let (x, _) = terminal_size().unwrap();
    let margin:usize = x as usize/2 - 27;
    let mut it = board.chars();
    let upper_case = String::from("ABCDEFGH");
    let mut rows = upper_case.chars();
    print!( "{}{}", " ".repeat(margin), " ".repeat(5));
    for column in String::from("12345678").chars(){
        print!( "   {}  ", column);
    }
    print!( "\n");
    for _ in 0..8{
        print!( "{}     ┼{}\n", " ".repeat(margin), "⎯⎯⎯⎯⎯┼".repeat(8));
        print!( "{}     |{}\n", " ".repeat(margin), "     ⎪".repeat(8));
        print!( "{}  {}  |", " ".repeat(margin), rows.next().unwrap());
        for _ in 0..8 {
            let cell = it.next().unwrap();
            match cell {
             'X' => print!( "  {}{}{}  ⎪",color::Fg(color::Red), cell, color::Fg(color::Reset)),
             'S' => print!( "  {}{}{}  ⎪",  color::Fg(color::Yellow), cell, color::Fg(color::Reset)),
             _ =>   print!( "  {}{}{}  ⎪",  color::Fg(color::LightBlue), cell, color::Fg(color::Reset)),
            }
        }
        print!( "\n");
        print!( "{}     |{}\n", " ".repeat(margin), "     ⎪".repeat(8));
    }
    print!( "{}     ┼{}\n", " ".repeat(margin), "⎯⎯⎯⎯⎯┼".repeat(8));
    print!( "\n");
    println!();
}


