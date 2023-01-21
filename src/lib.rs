use std::net::TcpStream;
use std::net::TcpListener;
use std::net::SocketAddr;
use std::io::Read;
use std::io::Write;
use termion::color;

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
// Cell::Shot => 'X',
// Cell::Ship => { match show_ships {
//    true => 'S',
//    false => 'O',
// }
// },
// Cell::Free => 'O',
pub fn print_board(board: String){
    let mut it = board.chars();
    let upper_case = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let mut rows = upper_case.chars();
    print!( "{}", " ".repeat(5));
    for column in String::from("12345678").chars(){
        print!( "   {}  ", column);
    }
    print!( "\n");
    for _ in 0..8{
        print!( "     ┼{}\n", "⎯⎯⎯⎯⎯┼".repeat(8));
        print!( "     |{}\n", "     ⎪".repeat(8));
        print!( "  {}  |", rows.next().unwrap());
        for _ in 0..8 {
            let cell = it.next().unwrap();
            match cell {
             'X' => print!( "  {}{}{}  ⎪", color::Fg(color::Red), cell, color::Fg(color::Reset)),
             'S' => print!( "  {}{}{}  ⎪", color::Fg(color::Yellow), cell, color::Fg(color::Reset)),
             _ =>   print!( "  {}{}{}  ⎪", color::Fg(color::LightBlue), cell, color::Fg(color::Reset)),
            }
        }
        print!( "\n");
        print!( "     |{}\n", "     ⎪".repeat(8));
    }
    print!( "     ┼{}\n", "⎯⎯⎯⎯⎯┼".repeat(8));
    print!( "\n");
    println!();
}
