
use casey_chess::board::Board;


fn main() {
    let mut board = Board::new();
    board.setup_starting_position();
    board.print(casey_chess::color::Color::White);
}
