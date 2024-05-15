
use casey_chess::{board::Board, utils::move_depth::depth_check};

fn console_game_loop() {
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

fn main() {
    // setup simple logger 
    simple_logger::SimpleLogger::new().with_colors(true).with_local_timestamps().init().unwrap();
    for i in 0..7 {
        let board = Board::starting_position();
        let start = std::time::Instant::now();
        let count = depth_check(i, board);
        let duration = start.elapsed();
        log::info!("Depth: {}, {} moves generated in {}ms", i, count, duration.as_millis());
    }
}

