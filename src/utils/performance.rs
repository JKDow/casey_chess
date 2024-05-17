use crate::board::Board;


pub fn perft(depth: u32, board: Board) -> usize {
    if depth == 0 {
        return 1;
    }
    let legal_moves = board.generate_legal_moves();
    let mut num_moves = 0;
    for mv in &legal_moves {
        let mut new_board = board.clone();
        if let Err(e) = new_board.move_piece(mv.clone()) {
            log::error!("Generated legal move flagged as illegal by move_piece: {}", e);
            new_board.print(crate::color::Color::White);
            log::trace!("Mv: {:?}", mv);
            std::process::exit(1);
        }
        num_moves += perft(depth - 1, new_board);
    }
    return num_moves;
}
