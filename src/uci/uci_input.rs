use super::{uci_commands::UciGuiToEngine, uci_messages::HandlerRx};


pub struct UciInput {
    tx: std::sync::mpsc::Sender<HandlerRx>,
}

impl UciInput {
    pub fn new(tx: std::sync::mpsc::Sender<HandlerRx>) -> UciInput {
        UciInput {
            tx,
        }
    }

    pub fn run_thread(mut self) -> std::thread::JoinHandle<()> {
        std::thread::spawn(move || self.main_loop())
    }

    fn main_loop(&mut self) {
        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            log::debug!("Received input: {}", input);
            let command = UciGuiToEngine::from_string(input);
            if let Some(command) = command {
                self.tx.send(HandlerRx::GuiMsg(command)).unwrap();
            }
        }
    }
}
