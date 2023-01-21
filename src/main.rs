use terminal_battle_ship::*;

fn main() {

                            // signal.store(false, Ordering::SeqCst);;
                            // thread.join().unwrap();
                    // let signal = Arc::new(AtomicBool::new(true));
                    // let receiver = Arc::clone(&signal);
                    // let (snd, rv) = mpsc::channel();
                    // let screen_borrow = screen_refcell.borrow_mut();
                    

                    // let thread = thread::spawn(move || {
                    //     rv.recv().unwrap();
                    //     let mut dots = 0;
                    //     while receiver.load(Ordering::SeqCst){
                    //        print!("\rWaiting{}{}", ".".repeat(dots + 1), " ".repeat(5-dots)); 
                    //        screen_borrow.flush().unwrap();
                    //        dots = (dots+1) % 4;
                    //        thread::sleep(std::time::Duration::new(1,0));
                    //     }
                    // });

    let board = String::from("OOOOOOOO\
                              OOOOOOOO\
                              OOOOOOOO\
                              OOOOOOOO\
                              OOOOOOOO\
                              OOOOOOOO\
                              OOOOOOOO\
                              OOOOOOOO\
                              ");

    println!("Board in Strin: {}", board);
    print_board(board);
}
