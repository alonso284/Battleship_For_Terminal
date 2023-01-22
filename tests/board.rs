use terminal_battle_ship::*;

#[test]
fn print_generic_board(){
    // 10 x 10 board;
    let board = String::from("OOOOOOOOOO\
                              OOOOOOOOOO\
                              OOXXXXOOOO\
                              OOOOOOOOOO\
                              OOOOOSOOOO\
                              OOOOOSOOOO\
                              OOOOOSOOOO\
                              OOOOOSOOOO\
                              OOOOOSOOOO\
                              OOOOOOOOOO");

    println!("Board as String: {}", board);
    print_board(&board);
}
