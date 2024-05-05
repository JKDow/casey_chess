use crate::{color::Color, piece_type::PieceType};

#[derive(Debug, PartialEq, Eq)]
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

    pub fn from_fen(fen: char) -> Option<Piece> {
        let color = if fen.is_uppercase() {
            Color::White
        } else {
            Color::Black
        };
        let piece = match fen.to_ascii_uppercase() {
            'P' => PieceType::Pawn,
            'R' => PieceType::Rook,
            'N' => PieceType::Knight,
            'B' => PieceType::Bishop,
            'Q' => PieceType::Queen,
            'K' => PieceType::King,
            _ => return None,
        };
        Some(Piece::new(piece, color))
    }
}
