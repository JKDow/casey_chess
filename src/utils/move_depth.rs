use crate::board::Board;


pub fn depth_check(depth: u32, board: Board) -> usize {
    if depth == 0 {
        return 1;
    }
    let legal_moves = board.generate_legal_moves();
    let mut num_moves = 0;
    for mv in &legal_moves {
        let mut new_board = board.clone();
        new_board.move_piece(mv.from_x, mv.from_y, mv.to_x, mv.to_y).unwrap();
        num_moves += depth_check(depth - 1, new_board);
    }
    return num_moves;
}
