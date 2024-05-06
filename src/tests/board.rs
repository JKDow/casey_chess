use crate::board::Board;


#[test]
fn move_piece_basic_1() {
    let mut board = Board::starting_position();
    board.move_piece(4, 1, 4, 3).unwrap();
    board.move_piece(4, 6, 4, 4).unwrap();
    board.move_piece(1, 0, 2, 2).unwrap();
    board.move_piece(1, 7, 2, 5).unwrap();

    let comp = Board::from_fen("r1bqkbnr/pppp1ppp/2n5/4p3/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 2 3").unwrap();
    let board_ref = board.get_squares();
    let comp_ref = comp.get_squares();
    for i in 0..8 {
        for j in 0..8 {
            assert_eq!(board_ref[i][j], comp_ref[i][j]);
        }
    }
}
