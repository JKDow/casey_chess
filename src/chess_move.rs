use std::fmt::Display;

use crate::piece_type::{self, PieceType};


#[derive(Debug, Clone)]
pub struct Move {
    pub from_x: usize,
    pub from_y: usize,
    pub to_x: usize,
    pub to_y: usize,
    pub piece_type: PieceType,
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let piece = if self.piece_type == PieceType::Pawn {String::new()} else { self.piece_type.to_string() };
        let file = |x| (b'a' + x as u8) as char;
        let rank = |y| (b'1' + y as u8) as char;
        write!(f, "{}{}{}{}{}", piece, file(self.from_x), rank(self.from_y), file(self.to_x), rank(self.to_y))
    }
}

