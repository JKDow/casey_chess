use casey_chess::{uci::uci_interface::UciHandler, utils::main_functions::console_game_loop};




fn main() {
    // setup simple logger 
    simple_logger::SimpleLogger::new()
        .with_colors(true)
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();
    UciHandler::new("Casey".to_string(), "JKDow".to_string()).run();
    //
    // console_game_loop();
}

