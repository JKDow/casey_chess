
use casey_chess::board::Board;


fn main() {
    // setup simple logger 
    simple_logger::SimpleLogger::new().with_colors(true).with_local_timestamps().init().unwrap();
    let mut board = Board::starting_position();
    board.print(casey_chess::color::Color::White);
    board.move_piece(4, 1, 4, 3).unwrap();
    board.print(casey_chess::color::Color::White);
    board.move_piece(4, 6, 4, 4).unwrap();
    board.print(casey_chess::color::Color::White);
    board.move_piece(1, 0, 2, 2).unwrap();
    board.print(casey_chess::color::Color::White);
    board.move_piece(1, 7, 2, 5).unwrap();
    board.print(casey_chess::color::Color::White);
}
