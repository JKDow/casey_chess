
use crate::{board::Board, piece_type::PieceType, utils::performance::perft};
use rand::Rng;

pub fn console_game_loop() {
    let mut board = Board::starting_position();
    board.print(crate::color::Color::White);

    loop {
        let moves = board.generate_legal_moves();
        if moves.len() == 0 {
            log::info!("Game over!\nWhite Wins!");
            break;
        }
        let mut input = String::new();
        loop {
            println!("Enter move: ");
            std::io::stdin().read_line(&mut input).unwrap();
            if let Err(e) = board.algebraic_move(input.trim()) {
                log::warn!("Invalid move: {}", e);
                input.clear();
                board.print(crate::color::Color::White);
            } else { break }
        }
        log::info!("White made move: {}", input);
        let moves = board.generate_legal_moves();
        if moves.len() == 0 {
            log::info!("Game over!\nWhite Wins!");
            break;
        }
        let mut rng = rand::thread_rng(); 
        let random_move = &moves[rng.gen_range(0..moves.len())];
        board.move_piece(random_move.clone()).unwrap();
        log::info!("Black made move: {}", random_move);
        board.print(crate::color::Color::White);
    }
}

pub fn depth_calc(depth: u32) {
    for i in 0..=depth {
        let board = Board::starting_position();
        let start = std::time::Instant::now();
        let count = perft(i, board);
        let duration = start.elapsed();
        log::info!("Depth: {}, {} moves generated in {}ms", i, count, duration.as_millis());
    }
}

pub fn perft_1() {
   for i in 0..=5 {
        let board = Board::from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 ").unwrap();
        let start = std::time::Instant::now();
        let count = perft(i, board);
        let duration = start.elapsed();
        log::info!("Depth: {}, {} moves generated in {}ms", i, count, duration.as_millis());
   }
}

pub fn perft_2() {
    let mut board = Board::from_fen("r3k2r/p1ppqpb1/bn2pnN1/3P4/1p2P3/2N2Q2/PPPBBPpP/R3K2R w KQkq - 0 2").unwrap();
    let mv = crate::chess_move::Move::new(6, 5, 7, 7, PieceType::Knight, None);
    board.move_piece(mv).unwrap();
    let start = std::time::Instant::now();
    let moves = board.generate_legal_moves();
    let mut count = 0;
    log::info!("{} moves generated", moves.len());
    for (i, mv) in moves.iter().enumerate() {
        let mut new_board = board.clone();
        new_board.move_piece(mv.clone()).unwrap();
        let n = perft(0, new_board);
        count += n;
        log::info!("{}| Move: {}, {} moves generated", i, mv, n);
    }
    let duration = start.elapsed();
    log::info!("{} moves generated in {}ms", count, duration.as_millis());
}
