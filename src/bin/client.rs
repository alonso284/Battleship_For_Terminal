use terminal_battle_ship::*;
use std::net::TcpStream;
use termion::screen::IntoAlternateScreen;
use std::io::Write;
use termion::color;

fn main(){
    // Get IP address of server
    let mut ip = String::new();
    print!("Enter the IP address of the server: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut ip).unwrap();
    let ip = ip.trim();

    // Connect to serevr
    let mut stream = match TcpStream::connect(format!("{}:{}", ip, PORT)){
        Ok(stream) => stream,
        Err(e) => panic!("Could to server in server in {}:{}: {}", ip, PORT, e),
    };
    println!("Successfully connected to server {}", rcv(&mut stream).unwrap());

    // Create fleet
    let my_fleet = Fleet::build_fleet();
    send(&mut stream, my_fleet.as_bytes()).unwrap();
    println!("Succesfully built fleet");
    println!("Waiting for other player to join");

    // Begin once both players are connected
    println!("All players have connected, beggining game {}", rcv(&mut stream).unwrap());
    send(&mut stream, b"TRUE").unwrap();
    println!("\n\nThe game is about to start");

    loop {
        // Get turn type form server
        let turn = rcv(&mut stream).unwrap();
        send(&mut stream, b"TRUE").unwrap();

        match &turn[..4] {
            // Play turn functionality
            turn if  turn == String::from("PLAY") => {
                loop {
                    // Get "ET" (End of Turn) and end turn or load the map
                    let cmd = rcv(&mut stream).unwrap() ;
                    send(&mut stream, b"TRUE").unwrap();
                    let mut screen = std::io::stdout().into_alternate_screen().unwrap();

                    match cmd[0..2] == String::from("ET") {
                        true => break,
                        false => { 
                            println!("Its you turn!");
                            println!("This is your opponents board, 'X' means that a place has been shot\n\n");
                            print_board(&cmd);
                        }
                    }
                    let _ = rcv(&mut stream).unwrap() ;
                    
                    // Get players cell to shoot
                    loop {
                        print!("Enter The cell you want to shoot [row][column]: ");
                        screen.flush().unwrap();
                        match get_play() {
                            Ok(coor) => {
                                send(&mut stream, coor.as_bytes()).unwrap();
                                println!("Sending turn, waiting for response");
                                break;
                            },
                            Err(e) => {
                                println!("Error while getting turn {}\n Try again", e);
                                continue;
                            },
                        }
                    }
                }
            },
            // Wait turn funcionality
            turn if turn == String::from("WAIT") => {
                loop {
                    // Get "ET" (End of Turn) and end turn or load the map
                    let cmd = rcv(&mut stream).unwrap() ;
                    send(&mut stream, b"TRUE").unwrap();
                    let mut screen = std::io::stdout().into_alternate_screen().unwrap();

                    match cmd[0..2] == String::from("ET") {
                        true => break,
                        false => {
                            println!("Take cover! It's the opponents turn");
                            println!("This is your board, the S's are your ships and the O are the places where your opponent has shot\n\n");
                            print_board(&cmd);
                        }
                    }
                    print!("Waiting for opponent to shoot");
                    screen.flush().unwrap();
                }
            },
            // If the turn is neither of the above, the game has ended
            status => {
                // End Game
                match status {
                    "LOST" => println!("Congratulations, you {}{}", color::Fg(color::Red), status),
                    _  => println!("Congratulations, you {}{}", color::Fg(color::Yellow), status),
                }
                println!("{}\n\n\n Ending program", color::Fg(color::Reset));
                std::thread::sleep(std::time::Duration::new(3,0));
                break;
            }
        }
    }
}
