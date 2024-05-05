
use casey_chess::board::Board;


fn main() {
    //let mut board = Board::starting_position();
    let mut board = Board::from_fen("rnbqkb1r/ppp2ppp/5n2/3pp3/4PP2/2N5/PPPP2PP/R1BQKBNR w KQkq d6 0 4").unwrap();
    board.print(casey_chess::color::Color::White);

    //board.unchecked_move_piece(0, 1, 0, 3);
    //board.print(casey_chess::color::Color::White);
}
