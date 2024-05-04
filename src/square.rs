use crate::{color::Color, piece::Piece};

pub struct Square {
    piece: Piece,
    color: Color,
}

impl Square {
    pub fn new(piece: Piece, color: Color) -> Square {
        Square { piece, color }
    }

    pub fn get_piece(&self) -> &Piece {
        &self.piece
    }

    pub fn get_color(&self) -> &Color {
        &self.color
    }

    // This function will return a character representing the piece on the square.
    // Upper case for white pieces, lower case for black pieces.
    pub fn get_piece_char(&self) -> char {
        let symbol = match self.piece {
            Piece::Pawn => 'P',
            Piece::Rook => 'R',
            Piece::Knight => 'N',
            Piece::Bishop => 'B',
            Piece::Queen => 'Q',
            Piece::King => 'K',
        };
        if self.color == Color::Black {
            symbol.to_lowercase().next().unwrap()
        } else {
            symbol
        }
    }
}
