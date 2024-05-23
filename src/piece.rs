use crate::{color::Color, move_type::MoveType, piece_type::PieceType};

#[derive(Clone, Debug, PartialEq, Eq)]
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

    pub fn is_white(&self) -> bool {
        self.color == Color::White
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

    /// Checks if the move is possible at all
    /// Simply checks if the given piece could theoretically move to the target square
    /// Does not check for check, pinned pieces, pieces in the way, etc.
    pub fn check_move(&self, from_x: usize, from_y:usize, to_x: usize, to_y: usize) -> MoveType {
        // check if the move is on the board
        // required to ensire as i8 cast is safe
        if from_x > 7 || from_y > 7 || to_x > 7 || to_y > 7 {
            return MoveType::Illegal;
        }
        match self.piece {
            PieceType::Pawn => self.is_legal_pawn_move(from_x as i8, from_y as i8, to_x as i8, to_y as i8),
            PieceType::Rook => self.is_legal_rook_move(from_x, from_y, to_x, to_y),
            PieceType::Knight => self.is_legal_knight_move(from_x, from_y, to_x, to_y),
            PieceType::Bishop => self.is_legal_bishop_move(from_x, from_y, to_x, to_y),
            PieceType::Queen => self.is_legal_queen_move(from_x, from_y, to_x, to_y),
            PieceType::King => self.is_legal_king_move(from_x, from_y, to_x, to_y),
        }
    }

    fn is_legal_pawn_move(&self, from_x: i8, from_y:i8, to_x: i8 , to_y: i8) -> MoveType {
        let direction: i8 = match self.color {
            Color::White => 1,
            Color::Black => -1,
        };
        let start_row: i8 = match self.color {
            Color::White => 1,
            Color::Black => 6,
        };
        if from_x == to_x {
            // Move forward
            if to_y == from_y + direction {
                return MoveType::Pawn1;
            } else if from_y == start_row && to_y == from_y + 2 * direction {
                return MoveType::Pawn2;
            }
        }
        else {
            // Capture move
            if to_y == from_y + direction && (to_x == from_x + 1 || to_x == from_x - 1) {
                return MoveType::PawnCapture;
            }
        }
        MoveType::Illegal
    }

    fn is_legal_rook_move(&self, from_x: usize, from_y:usize, to_x: usize, to_y: usize) -> MoveType {
        if from_x != to_x && from_y != to_y {
            return MoveType::Illegal;
        }
        MoveType::Rook
    }

    fn is_legal_knight_move(&self, from_x: usize, from_y:usize, to_x: usize, to_y: usize) -> MoveType {
        let x_diff = (to_x as i32 - from_x as i32).abs();
        let y_diff = (to_y as i32 - from_y as i32).abs();
        if (x_diff == 2 && y_diff == 1) || (x_diff == 1 && y_diff == 2){
            return MoveType::Knight;
        }
        MoveType::Illegal
    }

    fn is_legal_bishop_move(&self, from_x: usize, from_y:usize, to_x: usize, to_y: usize) -> MoveType {
        if (to_x as i32 - from_x as i32).abs() != (to_y as i32 - from_y as i32).abs() {
            return MoveType::Illegal;
        }
        MoveType::Bishop
    }

    fn is_legal_queen_move(&self, from_x: usize, from_y:usize, to_x: usize, to_y: usize) -> MoveType {
        if from_x != to_x && from_y != to_y {
            if (to_x as i32 - from_x as i32).abs() != (to_y as i32 - from_y as i32).abs() {
                return MoveType::Illegal;
            }
        }
        MoveType::Queen
    }

    fn is_legal_king_move(&self, from_x: usize, from_y:usize, to_x: usize, to_y: usize) -> MoveType {
        let x_diff = (to_x as i32 - from_x as i32).abs();
        let y_diff = (to_y as i32 - from_y as i32).abs();
        if x_diff <= 1 && y_diff <= 1 {
            return MoveType::KingNormal;
        }
        // check for castling
        if (self.is_white() && from_y == 0 && from_x == 4) || (!self.is_white() && from_y == 7 && from_x == 4) {
            if to_y == from_y && to_x == 6 {
                return MoveType::KingCastleKingSide;
            }
            if to_y == from_y && to_x == 2 {
                return MoveType::KingCastleQueenSide;
            }
        }        
        MoveType::Illegal
    }

    pub fn to_centipawns(&self) -> i32 {
        self.piece.to_centipawns()
    }
}
