use std::fmt::Display;
use crate::piece_type::PieceType;

#[derive(Debug, Clone)]
pub struct Move {
    pub from_x: usize,
    pub from_y: usize,
    pub to_x: usize,
    pub to_y: usize,
    pub piece_type: PieceType,
    pub promotion: Option<PieceType>,
}

impl Move {
    pub fn new(from_x: usize, from_y: usize, to_x: usize, to_y: usize, piece_type: PieceType, promotion: Option<PieceType>) -> Self {
        Move {
            from_x,
            from_y,
            to_x,
            to_y,
            piece_type,
            promotion,
        }
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let piece = if self.piece_type == PieceType::Pawn {String::new()} else { self.piece_type.to_string() };
        let file = |x| (b'a' + x as u8) as char;
        let rank = |y| (b'1' + y as u8) as char;
        if let Some(promotion) = &self.promotion {
            write!(f, "{}{}{}{}{}{}", piece, file(self.from_x), rank(self.from_y), file(self.to_x), rank(self.to_y), promotion)
        } else {
            write!(f, "{}{}{}{}{}", piece, file(self.from_x), rank(self.from_y), file(self.to_x), rank(self.to_y))
        }
    }
}

