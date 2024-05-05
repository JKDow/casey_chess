use crate::{color::Color, piece_type::PieceType};

pub struct Piece {
    piece: PieceType,
    color: Color,
}

impl Piece {
    pub fn new(piece: PieceType, color: Color) -> Piece {
        Piece { piece, color }
    }

    pub fn get_type(&self) -> &PieceType {
        &self.piece
    }

    pub fn get_color(&self) -> &Color {
        &self.color
    }

    // This function will return a character representing the piece on the square.
    // Upper case for white pieces, lower case for black pieces.
    pub fn get_piece_char(&self) -> char {
        let symbol = match self.piece {
            PieceType::Pawn => 'P',
            PieceType::Rook => 'R',
            PieceType::Knight => 'N',
            PieceType::Bishop => 'B',
            PieceType::Queen => 'Q',
            PieceType::King => 'K',
        };
        if self.color == Color::Black {
            symbol.to_lowercase().next().unwrap()
        } else {
            symbol
        }
    }
}
