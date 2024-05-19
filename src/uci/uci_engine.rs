use crate::{chess_move::Move, game::Game, piece_type::PieceType};

use super::uci_messages::{EngineMsg, HandlerRx, HandlerTx};

#[derive(Debug, PartialEq)]
enum UciEngineState {
    Idle,
    Running,
}

pub struct UciEngine {
    state: UciEngineState,
    rx: std::sync::mpsc::Receiver<HandlerTx>,
    tx: std::sync::mpsc::Sender<HandlerRx>,
    game: Game,
}

impl UciEngine {
    pub fn new(rx: std::sync::mpsc::Receiver<HandlerTx>, tx: std::sync::mpsc::Sender<HandlerRx>) -> UciEngine {
        UciEngine {
            state: UciEngineState::Idle,
            rx,
            tx,
            game: Game::new(),
        }
    }

    pub fn run_thread(mut self) -> std::thread::JoinHandle<()> {
        std::thread::spawn(move || self.main_loop())
    }

    fn main_loop(&mut self) {
        loop {
            let message = self.rx.recv().unwrap();
            match message {
                HandlerTx::NewFen(fen) => self.handle_new_fen(fen),
                HandlerTx::StartingPosition(moves) => self.handle_starting_position(moves),
                HandlerTx::StartSearch => self.handle_start_search(),
                HandlerTx::StopSearch => self.handle_stop_search(),
                HandlerTx::MakeMove(mv) => self.handle_make_move(mv),
            }
        }
    }

    fn handle_new_fen(&mut self, fen: String) {
        self.game = Game::from_fen(&fen);
        self.tx.send(HandlerRx::EngineMsg(EngineMsg::PositionSet)).unwrap();
    }

    fn handle_starting_position(&mut self, moves: String) {
        log::debug!("Setting starting position with moves: {}", moves);
        self.game = Game::new();
        let mut moves = moves.split_whitespace().collect::<Vec<&str>>();
        if moves.len() == 0 {
            self.tx.send(HandlerRx::EngineMsg(EngineMsg::PositionSet)).unwrap();
            return
        }
        if moves.remove(0) != "moves" {
            self.tx.send(HandlerRx::EngineMsg(EngineMsg::PositionSet)).unwrap();
            return 
        }
        for mv in moves {
            let from_x = mv.chars().nth(0).unwrap() as u8 - 97;
            let from_y = mv.chars().nth(1).unwrap() as u8 - 49;
            let to_x = mv.chars().nth(2).unwrap() as u8 - 97;
            let to_y = mv.chars().nth(3).unwrap() as u8 - 49;
            let promotion = if mv.len() == 5 {
                let piece = mv.chars().nth(4).unwrap();
                let piece = PieceType::try_from(piece).unwrap();
                Some(piece)
            } else {
                None
            };
            let piece = self.game.board.get_piece(from_x as usize, from_y as usize).unwrap();
            let mv: Move = Move::new(from_x as usize, from_y as usize, to_x as usize, to_y as usize, piece.get_type().clone(), promotion);
            self.game.make_move(mv).unwrap();
        }
        self.tx.send(HandlerRx::EngineMsg(EngineMsg::PositionSet)).unwrap();
    }

    fn handle_start_search(&mut self) {
        self.state = UciEngineState::Running;
        let mv = self.game.engine_move();
        self.tx.send(HandlerRx::EngineMsg(EngineMsg::FinalBestMove(mv.extended_algebraic()))).unwrap();
        self.state = UciEngineState::Idle; 
    }

    fn handle_stop_search(&mut self) {
        if self.state == UciEngineState::Idle {
            return
        }
        self.tx.send(HandlerRx::EngineMsg(EngineMsg::FinalBestMove("0000".to_string()))).unwrap();
        self.state = UciEngineState::Idle;
    }

    fn handle_make_move(&mut self, mv: String) {
        let mv = mv.trim();
        let from_x = mv.chars().nth(0).unwrap() as u8 - 97;
        let from_y = mv.chars().nth(1).unwrap() as u8 - 49;
        let to_x = mv.chars().nth(2).unwrap() as u8 - 97;
        let to_y = mv.chars().nth(3).unwrap() as u8 - 49;
        let promotion = if mv.len() == 5 {
            let piece = mv.chars().nth(4).unwrap().to_ascii_uppercase();
            let piece = PieceType::try_from(piece).unwrap();
            Some(piece)
        } else {
            None
        };
        let piece = self.game.board.get_piece(from_x as usize, from_y as usize).unwrap();
        let mv: Move = Move::new(from_x as usize, from_y as usize, to_x as usize, to_y as usize, piece.get_type().clone(), promotion);
        log::debug!("Engine is making move: {}", mv.extended_algebraic());
        self.game.make_move(mv).unwrap();
    }
}
