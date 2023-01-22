use terminal_battle_ship::*;
use local_ip_address::local_ip;

use std::net::TcpListener;

fn main(){
    // Use local IP to initiate server, only people in the local network will be able to connect ot
    // it unless you redirect you router in port 7878 to the device that runs the server
    let listener = match TcpListener::bind(format!("{:?}:{}", local_ip().unwrap(), PORT)){
        Ok(listener) => listener,
        Err(e) => panic!("Could initiate server in {:?}:{}: {}", local_ip().unwrap(), PORT, e),
    };
    println!("Running on server {:?}", local_ip().unwrap());

    let mut temp:String; 

    // Listen to players connections
    let mut player_one = connect_player(&listener).unwrap();
    let mut player_two = connect_player(&listener).unwrap();

    // Get player one's fleet configuration and load it
    let ship =  rcv(&mut player_one.socket).unwrap();
    temp = String::clone(&ship);
    player_one.build_fleet(ship.into_bytes());
    println!("Player 1 connected with setup: {}", temp);

    // Get player two's fleet configuration and load it
    let ship =  rcv(&mut player_two.socket).unwrap();
    temp = String::clone(&ship);
    player_two.build_fleet(ship.into_bytes());
    println!("Player 2 connected with setup: {}", temp);

    // Send confirmation two both players that the game is about to begin
    send(&mut player_one.socket, b"TRUE").unwrap();
    send(&mut player_two.socket, b"TRUE").unwrap();
    println!("Player 1 is ready to begin game {}", rcv(&mut player_one.socket).unwrap());
    println!("Player 2 is ready to begin game {}", rcv(&mut player_two.socket).unwrap());

    // Variable to toggle whose's turn it is to play
    let mut turn = true;
    loop {
        // Determine whose turn is it
        let (in_play, in_wait) = match turn {
             true => (&mut player_one, &mut player_two),
            false => (&mut player_two, &mut player_one),
        };
        // Begin turn
        handle_turn(in_play, in_wait);

        // End game if player in turn has lost all their ships
        if in_wait.is_sunk() {
            send(&mut in_play.socket, b"WON").unwrap();
            send(&mut in_wait.socket, b"LOST").unwrap();
            break;
        }

        turn = !turn;
    }

}

// Take care of players' inputs and give feedback on the other's players actions
fn handle_turn(in_play: &mut Player, in_wait: &mut Player){
    // Send the type of turn
    send(&mut in_play.socket, b"PLAY").unwrap();
    send(&mut in_wait.socket, b"WAIT").unwrap();
    println!("Player in play began turn: {}", rcv(&mut in_play.socket).unwrap());
    println!("Player in wait is idle: {}", rcv(&mut in_wait.socket).unwrap());

    loop {
        let covered_map = in_wait.board_str(false);
        let uncovered_map = in_wait.board_str(true );
        // Send in_wait's board to in_play player with the ships hidden
        send(&mut in_play.socket, covered_map.as_bytes()).unwrap();
        // Send in_wait's board to in_wait player with the ships it display
        send(&mut in_wait.socket, uncovered_map.as_bytes()).unwrap();
        println!("Player in play received board: {}", rcv(&mut in_play.socket).unwrap());
        println!("Player in wait received board: {}", rcv(&mut in_wait.socket).unwrap());

        // Get the turn of the player in_play
        // If it a miss of the opponents has no ships left, break; otherwise, continue
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

    // Endturn
    send(&mut in_play.socket, b"ET").unwrap();
    send(&mut in_wait.socket, b"ET").unwrap();
    println!("Player in play ended turn: {}", rcv(&mut in_play.socket).unwrap());
    println!("Player in wait ended turn: {}", rcv(&mut in_wait.socket).unwrap());
}
