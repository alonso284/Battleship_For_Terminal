use terminal_battle_ship::*;

use std::net::TcpListener;

fn main(){
    let listener = match TcpListener::bind(format!("{}:{}", IP, PORT)){
        Ok(listener) => listener,
        Err(e) => panic!("Could initiate server in {}:{}: {}", IP, PORT, e),
    };

    // CONN
    // BEGIN
    let mut player_one = connect_player(&listener).unwrap();
    println!("Player 1 connected {}", rcv(&mut player_one.socket).unwrap());
    let mut player_two = connect_player(&listener).unwrap();
    println!("Player 2 connected {}", rcv(&mut player_two.socket).unwrap());

    send(&mut player_one.socket, b"TRUE").unwrap();
    send(&mut player_two.socket, b"TRUE").unwrap();
    println!("Player 1 is ready to begin game {}", rcv(&mut player_one.socket).unwrap());
    println!("Player 2 is ready to begin game {}", rcv(&mut player_two.socket).unwrap());

    let mut turn = true;
    loop {
        // Determine whose turn is it
        let (in_play, in_wait) = match turn {
             true => (&mut player_one, &mut player_two),
            false => (&mut player_two, &mut player_one),
        };
        handle_turn(in_play, in_wait);

        if in_wait.is_sunk() {
            send(&mut in_play.socket, b"WON").unwrap();
            send(&mut in_wait.socket, b"LOST").unwrap();
            break;
        }

        turn = !turn;
    }

}

// TURN TYPE / STATUS
// BOARD
//      TURN
//      ...
// ET
fn handle_turn(in_play: &mut Player, in_wait: &mut Player){
    send(&mut in_play.socket, b"PLAY").unwrap();
    send(&mut in_wait.socket, b"WAIT").unwrap();
    println!("Player in play began turn: {}", rcv(&mut in_play.socket).unwrap());
    println!("Player in wait is idle: {}", rcv(&mut in_wait.socket).unwrap());

    loop {
        let covered_map = in_wait.board_str(false);
        let uncovered_map = in_wait.board_str(true );
        // Don't show ships
        send(&mut in_play.socket, covered_map.as_bytes()).unwrap();
        // Show ships
        send(&mut in_wait.socket, uncovered_map.as_bytes()).unwrap();
        println!("Player in play received board: {}", rcv(&mut in_play.socket).unwrap());
        println!("Player in wait received board: {}", rcv(&mut in_wait.socket).unwrap());

        send(&mut in_play.socket, b"TURN").unwrap();
        match in_wait.receive_shot(rcv(&mut in_play.socket).unwrap()) {
            Shot::Miss => break,
            Shot::Hit => {
                if in_wait.is_sunk(){
                    println!("Player in wait ships are sunk {}", in_wait.is_sunk());
                    break;
                }else{
                    continue;
                }
            },
        }
    }

    send(&mut in_play.socket, b"ET").unwrap();
    send(&mut in_wait.socket, b"ET").unwrap();
    println!("Player in play ended turn: {}", rcv(&mut in_play.socket).unwrap());
    println!("Player in wait ended turn: {}", rcv(&mut in_wait.socket).unwrap());
}
