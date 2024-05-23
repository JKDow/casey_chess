use casey_chess::{uci::uci_interface::UciHandler, utils::main_functions::console_game_loop};

//const LOG_LEVEL: log::LevelFilter = log::LevelFilter::Trace;
const LOG_LEVEL: log::LevelFilter = log::LevelFilter::Info;


fn main() {
    // setup simple logger 
    simple_logger::SimpleLogger::new()
        .with_colors(true)
        .with_level(LOG_LEVEL)
        .init()
        .unwrap();
    UciHandler::new("Casey".to_string(), "JKDow".to_string()).run();
}

