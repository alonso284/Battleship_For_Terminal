use std::net::TcpStream;
use std::net::TcpListener;
use std::net::SocketAddr;
use std::io;
use std::io::Read;
use std::io::Write;
use termion::color;
use termion::terminal_size;
use termion::screen::IntoAlternateScreen;

pub const PORT: &str = "7878";

// Server configurations
// Amount of ships
const CARRIER:usize = 1;
const BATTLESHIP:usize = 1;
const CRUISER:usize = 0;
const DESTROYER:usize = 0;

const N_SHIPS:usize = CARRIER*5 +  BATTLESHIP*4 + CRUISER*3 + DESTROYER*2;
const SHIP_COUNT:usize = CARRIER + BATTLESHIP + CRUISER + DESTROYER;

const WIDTH:usize = 10;
const HEIGHT:usize = 10;

pub struct Player {
    pub socket: TcpStream,
    pub address: SocketAddr,
    board: [[Cell;HEIGHT];WIDTH],
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
    // Create new default fleet
    pub fn new() -> Fleet {
        Fleet {
            carrier_count: CARRIER,
            battleship_count: BATTLESHIP,
            cruiser_count: CRUISER,
            destroyer_count: DESTROYER,
            board: String::from(format!("{}", "O".repeat(WIDTH*HEIGHT))),
            ships_locations: [None; 10],
            ships_placed: 0,
        }
    }
    // Add ship to fleet
    pub fn add_ship(&mut self, ship_type: Ships, (tx, ty):(usize, usize), orientation: Orientation) -> Result<(), String> {
        println!("Building ship");
        let position = Position {
            x: tx, y: ty, orientation,
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
            Orientation::Vertical => position.x + free_space - 1 > HEIGHT,
            Orientation::Horizontal => position.y + free_space - 1 > WIDTH,
        } == true {
            return Err(String::from("There is no enough space to place ship"));
        }

        // Create an array of bytes to update the cell
        let mut board_bytes = [0u8;WIDTH*HEIGHT];
        let mut iterator = self.board.bytes();
        for i in 0..WIDTH*HEIGHT{
            board_bytes[i] = iterator.next().unwrap();
        }

        let mut x = position.x;
        let mut y = position.y;

        // Check if there are no ships in the way
        for _ in 0..free_space {
            let cell = x*WIDTH + y;
            if let b'S' =  board_bytes[cell] {
                return Err(String::from("One of the cells has been already occupied"));
            }
            match position.orientation {
                Orientation::Vertical => x += 1,
                Orientation::Horizontal => y += 1,
            }
        }
        
        // Create updated version of the board once everything is cleared
        let mut x = position.x;
        let mut y = position.y;
        for _ in 0..free_space {
            let cell = x*WIDTH + y;
            board_bytes[cell] = b'S';
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
        // Update board
        self.board = String::from_utf8_lossy(&board_bytes).to_string();

        Ok(())
    }
    // Get unmutable borrow of the board string
    pub fn get_board(&self) -> &String {
        &self.board
    }
    // Print the locations of the ships that have been placed
    pub fn print_status(&self){
        for ship in self.ships_locations{
            match ship {
            Some(ship) => println!("There is a {:?} in position ({}, {}) with orientation {:?}", ship.1, ship.0.x, ship.0.y, ship.0.orientation),
            None => {},
            }
        }
    }
    // Returns whether all ships have been placed
    pub fn all_ships_placed(&self) -> bool {
        self.ships_placed == SHIP_COUNT
    }
    // Get the ships that still need to be placed
    pub fn get_quantities(&self) -> Vec<(Ships, usize)> {
        vec![   (Ships::Carrier, self.carrier_count),
                (Ships::Battleship, self.battleship_count),
                (Ships::Cruiser, self.cruiser_count),
                (Ships::Destroyer, self.destroyer_count), ]
    }
    // Automate creation of fleet
    pub fn build_fleet() -> String {
        // Create new fleet
        let mut fleet = Fleet::new();

        // Run until all ships have been placed
        while !fleet.all_ships_placed() {
            let mut screen = std::io::stdout().into_alternate_screen().unwrap();
            println!("Lets set up your fleet\n\n\n\n");
            print_board(fleet.get_board());

            // Print the ships that are yet to be placed
            let quantities = fleet.get_quantities();
            for ship in quantities {
                print!("{} of type {:?}\t\t", ship.1, ship.0);
            }
            println!();

            // Get ship type
            let ship:Ships;
            print!("Which ship would you like to place?: ");
            screen.flush().unwrap();
            loop {
                let mut input_ship = String::new();
                std::io::stdin().read_line(&mut input_ship).unwrap();
                input_ship.pop();
                if input_ship == String::from("Carrier"){
                    ship = Ships::Carrier;
                    break;
                } 
                else
                if input_ship == String::from("Battleship"){
                    ship = Ships::Battleship;
                    break;
                }
                else
                if input_ship == String::from("Cruiser"){
                    ship = Ships::Cruiser;
                    break;
                }
                else
                if input_ship == String::from("Destroyer"){
                    ship = Ships::Destroyer;
                    break;
                }
                print!("That ship is not available, try again: ");
                screen.flush().unwrap();
            }

            // Get the placing cell
            print!("Enter The cell you want to shoot [row][column]: ");
            screen.flush().unwrap();
            let coor = 
            loop {
                match get_play() {
                    Ok(coor) => {
                        break parse_turn(coor);
                    },
                    Err(e) => {
                        println!("Error while getting turn {}\n Try again", e);
                        continue;
                    },
                }
            };

            
            // Get orientation of ship
            let orientation:Orientation;
            print!("Which orientation would you like to place your ship in (Vertical|Horizontal)?: ");
            screen.flush().unwrap();
            loop {
                let mut input_orientation = String::new();
                std::io::stdin().read_line(&mut input_orientation).unwrap();
                input_orientation.pop();
                if input_orientation == String::from("Vertical"){
                    orientation = Orientation::Vertical;
                    break;
                } 
                else
                if input_orientation == String::from("Horizontal"){
                    orientation = Orientation::Horizontal;
                    break;
                }
                print!("That orientation does not exist, try again: ");
                screen.flush().unwrap();
            }

            // Add the ship to the fleet
            if let Err(e) = fleet.add_ship(ship, coor, orientation) {
                println!("Error while adding ship: {}. Press ENTER to continue", e);
                std::io::stdin().read_line(&mut String::new()).unwrap(); 
            }
        }

        // Return the configuration of the fleet as a string
        fleet.board
    }
}

// Get the size of a ship
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
    // Generate new player
    pub fn new(socket: TcpStream, address: SocketAddr) -> Player{
        let board = [[Cell::Free;HEIGHT];WIDTH];
        Player{
            socket,
            address,
            board,
            ship_units_left: N_SHIPS,
        }
    }
    // Build fleet from a vector of bytes 
    pub fn build_fleet(&mut self, template: Vec<u8>){
        for x in 0..HEIGHT {
            for y in 0..WIDTH {
                if let b'S' = template[x*WIDTH + y] {
                    self.board[x][y] = Cell::Ship;
                }
            }
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
       
        let (x, y) = parse_turn(turn);
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
}

// Turn turn string to tuple
pub fn parse_turn(turn:String) -> (usize, usize){
    let mut it = turn.chars();
    ((it.next().unwrap() as u8 - 'A' as u8) as usize, (it.next().unwrap() as u8 - '0' as u8) as usize)
}
// Get String of players cell to shoot
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

    if x < 'A' || 'J' < x{
        return Err(String::from(format!("First coordinate must be between A and {}", (('A' as u8 + HEIGHT as u8 - 1) as char))));
    }
    if y < '0' || '9' < y{
        return Err(String::from(format!("Second coordinate must be between 1 and {}", (('0' as u8 + WIDTH as u8 - 1) as char))));
    }

    Ok(play.to_string())
}
// send message to stream
pub fn send(stream:&mut TcpStream, msg: &[u8]) -> Result<(), String> {
    stream.write(msg).unwrap();
    stream.flush().unwrap();
    // println!("Sending message: {}", String::from_utf8_lossy(msg).to_string());
    Ok(())
}
// receive message from stream
pub fn rcv(stream:&mut TcpStream) -> Result<String, String>{
    let mut buffer = [0u8;1024];
    stream.read(&mut buffer).unwrap();
    // println!("Recieving message: {}", String::from_utf8_lossy(&buffer));
    Ok(String::from_utf8_lossy(&buffer).to_string())
}
// Listen to players connection and send confirmation message
pub fn connect_player(listener: &TcpListener) -> Result<Player, std::io::Error> {
    let mut player = match listener.accept() {
        Ok((socket, address)) => Player::new(socket, address),
        Err(e) => return Err(e),
    };

    send(&mut player.socket, b"TRUE").unwrap();

    println!("Player connected in {:?}", player.address);

    Ok(player)
}
// Print board from string borrow with special formate
pub fn print_board(board: &String){
    let (x, _) = terminal_size().unwrap();
    let margin:usize = x as usize/2 - (WIDTH+1)*6/2;
    let mut it = board.chars();
    let upper_case = String::from("ABCDEFGHIJ");
    let mut rows = upper_case.chars();
    print!( "{}{}", " ".repeat(margin), " ".repeat(5));
    for column in String::from("0123456789").chars(){
        print!( "   {}  ", column);
    }
    print!( "\n");
    for _ in 0..HEIGHT{
        print!( "{}     ┼{}\n", " ".repeat(margin), "⎯⎯⎯⎯⎯┼".repeat(WIDTH));
        print!( "{}     |{}\n", " ".repeat(margin), "     ⎪".repeat(WIDTH));
        print!( "{}  {}  |", " ".repeat(margin), rows.next().unwrap());
        for _ in 0..WIDTH {
            let cell = it.next().unwrap();
            match cell {
             'X' => print!( "  {}{}{}  ⎪",color::Fg(color::Red), cell, color::Fg(color::Reset)),
             'S' => print!( "  {}{}{}  ⎪",  color::Fg(color::Yellow), cell, color::Fg(color::Reset)),
             _ =>   print!( "  {}{}{}  ⎪",  color::Fg(color::LightBlue), cell, color::Fg(color::Reset)),
            }
        }
        print!( "\n");
        print!( "{}     |{}\n", " ".repeat(margin), "     ⎪".repeat(WIDTH));
    }
    print!( "{}     ┼{}\n", " ".repeat(margin), "⎯⎯⎯⎯⎯┼".repeat(WIDTH));
    print!( "\n");
    println!();
}
