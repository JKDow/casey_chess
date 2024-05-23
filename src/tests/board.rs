use crate::utils::performance::perft;
use crate::{board::Board, chess_move::Move};
use crate::piece_type::PieceType;


#[test]
fn move_piece_basic_1() {
    let mut board = Board::starting_position();
    board.move_piece(Move::new(4,1,4,3, PieceType::Pawn, None)).unwrap();
    board.move_piece(Move::new(4,6,4,4, PieceType::Pawn, None)).unwrap();
    board.move_piece(Move::new(1,0,2,2, PieceType::Knight, None)).unwrap();
    board.move_piece(Move::new(1,7,2,5, PieceType::Knight, None)).unwrap();

    let comp = Board::from_fen("r1bqkbnr/pppp1ppp/2n5/4p3/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 2 3").unwrap();
    let board_ref = board.get_squares();
    let comp_ref = comp.get_squares();
    for i in 0..8 {
        for j in 0..8 {
            assert_eq!(board_ref[i][j], comp_ref[i][j]);
        }
    }
}

#[test]
fn moves_from_start_1() {
    let board = Board::starting_position();
    let count = perft(1, board);
    assert_eq!(count, 20);
}

#[test]
fn moves_from_start_2() {
    let board = Board::starting_position();
    let count = perft(2, board);
    assert_eq!(count, 400);
}
