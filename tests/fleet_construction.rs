use terminal_battle_ship::*;

#[test]
fn add_ships(){
    let mut fleet = Fleet::new();
    println!("{:?}", fleet);
    let ships = vec![   (Ships::Carrier, (1,1), Orientation::Vertical),
                        (Ships::Battleship, (1,2), Orientation::Vertical),
                        (Ships::Battleship, (1,3), Orientation::Vertical),
                        (Ships::Cruiser, (1,4), Orientation::Vertical),
                        (Ships::Cruiser, (1,5), Orientation::Vertical),
                        (Ships::Cruiser, (1,6), Orientation::Vertical),
                        (Ships::Destroyer, (8,1), Orientation::Horizontal),
                        (Ships::Destroyer, (8,3), Orientation::Horizontal),
                        (Ships::Destroyer, (8,5), Orientation::Horizontal),
                        (Ships::Destroyer, (8,7), Orientation::Horizontal), ];
    for ship in ships {
        if let Err(e) = fleet.add_ship(ship.0, ship.1, ship.2) {
            println!("Error while adding ship: {}", e);
        }
    }
    print_board(fleet.get_board());
    fleet.print_status();
    println!("{:?}", fleet);
    assert_eq!(true, fleet.all_ships_placed());
}

#[test]
fn stop_from_ship_count(){
    let mut fleet = Fleet::new();
    fleet.add_ship(Ships::Carrier, (1,1), Orientation::Vertical).unwrap();
    if let Err(e) = fleet.add_ship(Ships::Carrier, (1,2), Orientation::Vertical) {
        println!("Error while adding ship: {}", e);
        assert_eq!(e, "No Carriers left");
    }
}

#[test]
fn stop_from_out_of_bound(){
    let mut fleet = Fleet::new();
    if let Err(e) = fleet.add_ship(Ships::Carrier, (7,1), Orientation::Vertical) {
        println!("Error while adding ship: {}", e);
        assert_eq!(e, "There is no enough space to place ship");
    }
}

#[test]
fn stop_from_ship_in_the_way(){
    let mut fleet = Fleet::new();
    fleet.add_ship(Ships::Battleship, (1,5), Orientation::Vertical).unwrap();
    if let Err(e) = fleet.add_ship(Ships::Carrier, (3,1), Orientation::Horizontal) {
        println!("Error while adding ship: {}", e);
        assert_eq!(e, "One of the cells has been already occupied");
    }
}
