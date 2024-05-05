use crate::{color::Color, piece_type::PieceType, piece::Piece};

pub struct Board {
    squares: Vec<Vec<Option<Piece>>>
}

impl Board {
    pub fn new() -> Board {
        let mut squares = Vec::new();
        for _ in 0..8 {
            let mut row = Vec::new();
            for _ in 0..8 {
                row.push(None);
            }
            squares.push(row);
        }
        Board { squares }
    }

    pub fn set_piece(&mut self, x: usize, y: usize, piece: PieceType, color: Color) {
        self.squares[y][x] = Some(Piece::new(piece, color));
    }

    /// Move a pice from one square to another.
    /// Returns the piece that was taken, if any.
    pub fn unchecked_move_piece(&mut self, from_x: usize, from_y: usize, to_x: usize, to_y: usize) -> Option<Piece> {
        let piece = self.squares[from_y][from_x].take();
        let taken_piece = self.squares[to_y][to_x].take();
        self.squares[to_y][to_x] = piece;
        taken_piece
    }

    pub fn setup_starting_position(&mut self) {
        for i in 0..8 {
            self.set_piece(i, 1, PieceType::Pawn, Color::White);
            self.set_piece(i, 6, PieceType::Pawn, Color::Black);
        }
        self.set_piece(0, 0, PieceType::Rook, Color::White);
        self.set_piece(1, 0, PieceType::Knight, Color::White);
        self.set_piece(2, 0, PieceType::Bishop, Color::White);
        self.set_piece(3, 0, PieceType::Queen, Color::White);
        self.set_piece(4, 0, PieceType::King, Color::White);
        self.set_piece(5, 0, PieceType::Bishop, Color::White);
        self.set_piece(6, 0, PieceType::Knight, Color::White);
        self.set_piece(7, 0, PieceType::Rook, Color::White);

        self.set_piece(0, 7, PieceType::Rook, Color::Black);
        self.set_piece(1, 7, PieceType::Knight, Color::Black);
        self.set_piece(2, 7, PieceType::Bishop, Color::Black);
        self.set_piece(3, 7, PieceType::Queen, Color::Black);
        self.set_piece(4, 7, PieceType::King, Color::Black);
        self.set_piece(5, 7, PieceType::Bishop, Color::Black);
        self.set_piece(6, 7, PieceType::Knight, Color::Black);
        self.set_piece(7, 7, PieceType::Rook, Color::Black);
    }

    pub fn print(&self, perspective: Color) {
        let (column_label, rows, columns) = if perspective == Color::White {
            ("    a   b   c   d   e   f   g   h", (0..8).rev().collect::<Vec<_>>(), (0..8).collect::<Vec<_>>())
        } else {
            ("    h   g   f   e   d   c   b   a", (0..8).collect::<Vec<_>>(), (0..8).rev().collect::<Vec<_>>())
        };
        println!("{}", column_label); 

        for i in &rows {
            println!("  +---+---+---+---+---+---+---+---+");
            let row_label = i + 1;
            print!("{} ", row_label);

            for j in &columns {
                print!("| ");
                let symbol = match &self.squares[*i][*j] {
                    Some(piece) => piece.get_piece_char().to_string(),
                    None => " ".to_string(),
                };
                print!("{} ", symbol);
            }
            println!("| {}", row_label);
        }
        println!("  +---+---+---+---+---+---+---+---+");
        println!("{}", column_label); 
    }
}
