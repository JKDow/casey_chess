use super::{uci_commands::{UciEngineToGui, UciGuiToEngine}, uci_engine::UciEngine, uci_input::UciInput, uci_messages::{EngineMsg, HandlerRx, HandlerTx}};

#[derive(Debug, PartialEq)]
enum UciHandlerState {
    New, // Just created
    Ready, // Has received the "uci" command
    Idle, // Has received a position
    Thinking, // Is calculating a move
    WaitMsg, // Waiting for a message from a thread
}

pub struct UciHandler {
    pub name: String,
    pub author: String,
    state: UciHandlerState,
    tx: std::sync::mpsc::Sender<HandlerTx>,
    rx: std::sync::mpsc::Receiver<HandlerRx>,
    _engine_handle: std::thread::JoinHandle<()>,
    _input_handle: std::thread::JoinHandle<()>,
    current_best_move: Option<String>,
}

impl UciHandler {
    pub fn new(name: String, author: String) -> UciHandler {
        let (handler_tx, engine_rx) = std::sync::mpsc::channel();
        let (engine_tx, handler_rx) = std::sync::mpsc::channel();
        let engine = UciEngine::new(engine_rx, engine_tx.clone());
        let engine_handle = engine.run_thread();
        let input_handler = UciInput::new(engine_tx);
        let input_handle = input_handler.run_thread();
        UciHandler {
            name,
            author,
            state: UciHandlerState::New,
            tx: handler_tx,
            rx: handler_rx,
            _engine_handle: engine_handle,
            _input_handle: input_handle,
            current_best_move: None,
        }
    }

    pub fn run(&mut self) {
        loop {
            let message = self.rx.recv().unwrap();
            match message {
                HandlerRx::EngineMsg(msg) => self.handle_engine_message(msg),
                HandlerRx::GuiMsg(input) => self.handle_input(input),
            }
        }
    }
    
    fn handle_engine_message(&mut self, message: EngineMsg) {
        log::debug!("Received engine message: {:?}", message);
        match message {
            EngineMsg::PositionSet => {
                self.state = UciHandlerState::Idle;
            },
            EngineMsg::CurrentBestMove(mv) => {
                self.current_best_move = Some(mv);
            },
            EngineMsg::FinalBestMove(mv) => {
                self.send_command(UciEngineToGui::best_move(&mv));
                self.state = UciHandlerState::Idle;
            },
        }
    }

    fn handle_input(&mut self, input: UciGuiToEngine) {
        log::debug!("Received input: {:?}", input);
        match input {
            UciGuiToEngine::Uci => self.command_uci(),
            UciGuiToEngine::IsReady => self.command_isready(),
            UciGuiToEngine::Position(pos) => self.command_position(&pos),
            UciGuiToEngine::Go(options) => self.command_go(&options),
            UciGuiToEngine::Stop => self.command_stop(),
            UciGuiToEngine::Quit => self.command_quit(),
            _ => {},
        }
    }

    fn command_uci(&mut self) {
        log::debug!("Received UCI command");
        log::trace!("Current state: {:?}", self.state);
        if self.state != UciHandlerState::New {
            return;
        }
        self.send_command(UciEngineToGui::id_name(&self.name));
        self.send_command(UciEngineToGui::id_author(&self.author));
        self.send_command(UciEngineToGui::uci_ok());
        self.state = UciHandlerState::Ready;
    }

    fn command_isready(&self) {
        self.send_command(UciEngineToGui::ready_ok());
    }

    fn command_position(&mut self, pos: &str) {
        match self.state {
            UciHandlerState::New => {}
            UciHandlerState::Ready => {
                let parts: Vec<&str> = pos.split_whitespace().collect(); 
                if parts[0] == "startpos" {
                    self.tx.send(HandlerTx::StartingPosition(parts[1..].join(" "))).unwrap();
                } else if parts[0] == "fen" {
                    self.tx.send(HandlerTx::NewFen(parts[1..].join(" "))).unwrap();
                }
                self.state = UciHandlerState::WaitMsg;
            }
            UciHandlerState::Idle => {
                let parts: Vec<&str> = pos.trim().split_whitespace().collect(); 
                let mv = parts.last().unwrap().to_string();
                log::debug!("Got move {} from parts {:?}", mv, parts);
                self.tx.send(HandlerTx::MakeMove(mv)).unwrap();
            }
            UciHandlerState::Thinking => {}
            UciHandlerState::WaitMsg => {}
        }
    }

    fn command_go(&mut self, _options: &str) {
        if self.state != UciHandlerState::Idle {
            return;
        }
        self.tx.send(HandlerTx::StartSearch).unwrap();
        self.state = UciHandlerState::Thinking;
    }

    fn command_stop(&mut self) {
        if self.state != UciHandlerState::Thinking {
            return;
        }
        let mv = match self.current_best_move.take() {
            Some(mv) => mv,
            None => "0000".to_string(), // this is an invalid move
        };
        self.send_command(UciEngineToGui::best_move(&mv));
        self.tx.send(HandlerTx::StopSearch).unwrap();
        self.state = UciHandlerState::Idle;
    }

    fn command_quit(&self) {
        std::process::exit(0);
    }

    fn send_command(&self, command: UciEngineToGui) {
        log::debug!("Sending command: {}", command);
        println!("{}", command);
    }
}
