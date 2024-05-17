use rand::Rng;

use crate::{board::Board, chess_move::{self, Move}, color::Color, errors::move_error::MoveError};

pub struct Game {
    pub board: Board,
    pub move_history_white: Vec<chess_move::Move>,
    pub move_history_black: Vec<chess_move::Move>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::starting_position(),
            move_history_white: Vec::new(),
            move_history_black: Vec::new(),
        }
    }

    pub fn make_move(&mut self, mv: chess_move::Move) -> Result<(), MoveError> {
        self.board.move_piece(mv.clone())?;
        match self.board.get_player_turn() {
            Color::White => self.move_history_white.push(mv),
            Color::Black => self.move_history_black.push(mv),
        }
        Ok(())
    }

    pub fn engine_move(&mut self) -> Move {
        let color = self.board.get_player_turn().clone();
        let moves = self.board.generate_legal_moves();
        let mut rng = rand::thread_rng(); 
        let random_move = &moves[rng.gen_range(0..moves.len())];
        self.board.move_piece(random_move.clone()).unwrap();
        match color {
            Color::White => self.move_history_white.push(random_move.clone()),
            Color::Black => self.move_history_black.push(random_move.clone()),
        }
        return random_move.clone();
    }

    pub fn from_fen(fen: &str) -> Game {
        Game {
            board: Board::from_fen(fen).unwrap(),
            move_history_white: Vec::new(),
            move_history_black: Vec::new(),
        }
    }
}
