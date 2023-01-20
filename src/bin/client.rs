use terminal_battle_ship::*;
use std::net::TcpStream;

// CONN
// BEGIN
// TURN TYPE / STATUS
// BOARD
//      TURN
//      ...
// ET
fn main(){
    let mut stream = match TcpStream::connect(format!("{}:{}", IP, PORT)){
        Ok(stream) => stream,
        Err(e) => panic!("Could to server in server in {}:{}: {}", IP, PORT, e),
    };

    println!("Successfully connected to server {}", rcv(&mut stream).unwrap());
    send(&mut stream, b"TRUE").unwrap();
    println!("All players have connected, beggining game {}", rcv(&mut stream).unwrap());
    send(&mut stream, b"TRUE").unwrap();

    loop {
        let turn = rcv(&mut stream).unwrap();
        send(&mut stream, b"TRUE").unwrap();

        println!("{} {:?} {}", turn, &turn[..4], String::from("PLAY") == turn);
        match &turn[..4] {
            turn if  turn == String::from("PLAY") => {
                loop {
                    let cmd = rcv(&mut stream).unwrap() ;
                    send(&mut stream, b"TRUE").unwrap();

                    match cmd[0..2] == String::from("ET") {
                        true => break,
                        false => print_board(cmd),
                    }

                   let _ = rcv(&mut stream).unwrap() ;
                    
                    loop {
                        match get_play() {
                            Ok(coor) => {
                                send(&mut stream, coor.as_bytes()).unwrap();
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

                    match cmd[0..2] == String::from("ET") {
                        true => break,
                        false => print_board(cmd),
                    }
                }
            },
            status => {
                println!("Congratulations, you {}", status);
                break;
            }
        }

    }
}
