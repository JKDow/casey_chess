
use casey_chess::board::Board;


fn main() {
    // setup simple logger 
    simple_logger::SimpleLogger::new().with_colors(true).with_local_timestamps().init().unwrap();
    let mut board = Board::starting_position();
    board.print(casey_chess::color::Color::White);
    loop {
        let mut input = String::new();
        println!("Enter move: ");
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if let Err(e) = board.algebraic_move(input) {
            log::warn!("Invalid move: {}", e);
        }
        board.print(casey_chess::color::Color::White);
    }
}
