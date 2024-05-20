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
        let mut best_move_index = 0;
        let mut best_move_score = self.board.evaluate_move(moves[0].clone()).unwrap();  
        for i in 1..moves.len() {
            let score = self.board.evaluate_move(moves[i].clone()).unwrap();
            if score > best_move_score {
                best_move_score = score;
                best_move_index = i;
            }
        }
        let best_move = moves[best_move_index].clone();
        self.board.move_piece(best_move.clone()).unwrap();
        log::trace!("Engine made move for it's turn: {}", best_move.extended_algebraic());
        match color {
            Color::White => self.move_history_white.push(best_move.clone()),
            Color::Black => self.move_history_black.push(best_move.clone()),
        }
        return best_move;
    }

    pub fn from_fen(fen: &str) -> Game {
        Game {
            board: Board::from_fen(fen).unwrap(),
            move_history_white: Vec::new(),
            move_history_black: Vec::new(),
        }
    }
}
