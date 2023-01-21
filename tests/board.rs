use terminal_battle_ship::*;

#[test]
fn print_generic_board(){
        let board = String::from("OOOOOOOO\
                              OOOOOOOO\
                              OOXXXXOO\
                              OOOOOOOO\
                              OOOOOSOO\
                              OOOOOSOO\
                              OOOOOSOO\
                              OOOOOOOO\
                              ");

    println!("Board as String: {}", board);
    print_board(&board);
}
