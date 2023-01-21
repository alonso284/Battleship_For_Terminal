use terminal_battle_ship::*;
use std::net::TcpStream;
use termion::screen::IntoAlternateScreen;
use std::io::Write;
use termion::color;

fn main(){
    let mut stream = match TcpStream::connect(format!("{}:{}", IP, PORT)){
        Ok(stream) => stream,
        Err(e) => panic!("Could to server in server in {}:{}: {}", IP, PORT, e),
    };

    // let mut screen = std::io::stdout().into_alternate_screen().unwrap();

    println!("Successfully connected to server {}", rcv(&mut stream).unwrap());
    send(&mut stream, b"TRUE").unwrap();
    println!("All players have connected, beggining game {}", rcv(&mut stream).unwrap());
    send(&mut stream, b"TRUE").unwrap();
    println!("\n\nThe game is about to start");
    // screen.flush().unwrap();
    std::thread::sleep(std::time::Duration::new(3,0));

    loop {
        let turn = rcv(&mut stream).unwrap();
        send(&mut stream, b"TRUE").unwrap();

        match &turn[..4] {
            turn if  turn == String::from("PLAY") => {
                loop {
                    let cmd = rcv(&mut stream).unwrap() ;
                    send(&mut stream, b"TRUE").unwrap();
                    let mut screen = std::io::stdout().into_alternate_screen().unwrap();
                    println!("Its you turn!");

                    match cmd[0..2] == String::from("ET") {
                        true => break,
                        false => { 
                            println!("This is your opponents board, 'X' means that a place has been shot\n\n");
                            print_board(cmd);
                        }
                    }

                   let _ = rcv(&mut stream).unwrap() ;
                    
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
            turn if turn == String::from("WAIT") => {
                loop {
                    let cmd = rcv(&mut stream).unwrap() ;
                    send(&mut stream, b"TRUE").unwrap();
                    let mut screen = std::io::stdout().into_alternate_screen().unwrap();
                    println!("Take cover! It's the opponents turn");

                    match cmd[0..2] == String::from("ET") {
                        true => break,
                        false => {
                            println!("This is your board, the S's are your ships and the O are the places where your opponent has shot\n\n");
                            print_board(cmd);
                        }
                    }
                    print!("Waiting for opponent to shoot");
                    screen.flush().unwrap();
                }
            },
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
