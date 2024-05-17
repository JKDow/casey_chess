
use casey_chess::{board::Board, utils::move_depth::depth_check};
use rand::Rng;

fn console_game_loop() {
    let mut board = Board::starting_position();
    board.print(casey_chess::color::Color::White);

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
                board.print(casey_chess::color::Color::White);
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
        board.print(casey_chess::color::Color::White);
    }
}

fn depth_calc(depth: u32) {
    for i in 0..=depth {
        let board = Board::starting_position();
        let start = std::time::Instant::now();
        let count = depth_check(i, board);
        let duration = start.elapsed();
        log::info!("Depth: {}, {} moves generated in {}ms", i, count, duration.as_millis());
    }
}

fn main() {
    // setup simple logger 
    simple_logger::SimpleLogger::new().with_colors(true).with_local_timestamps().init().unwrap();
    //console_game_loop();
    depth_calc(5);
}

