use crate::{square::Square, color::Color, piece::Piece};

pub struct Board {
    squares: Vec<Vec<Option<Square>>>
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

    pub fn set_piece(&mut self, x: usize, y: usize, piece: Piece, color: Color) {
        self.squares[y][x] = Some(Square::new(piece, color));
    }

    pub fn setup_starting_position(&mut self) {
        for i in 0..8 {
            self.set_piece(i, 1, Piece::Pawn, Color::White);
            self.set_piece(i, 6, Piece::Pawn, Color::Black);
        }
        self.set_piece(0, 0, Piece::Rook, Color::White);
        self.set_piece(1, 0, Piece::Knight, Color::White);
        self.set_piece(2, 0, Piece::Bishop, Color::White);
        self.set_piece(3, 0, Piece::Queen, Color::White);
        self.set_piece(4, 0, Piece::King, Color::White);
        self.set_piece(5, 0, Piece::Bishop, Color::White);
        self.set_piece(6, 0, Piece::Knight, Color::White);
        self.set_piece(7, 0, Piece::Rook, Color::White);

        self.set_piece(0, 7, Piece::Rook, Color::Black);
        self.set_piece(1, 7, Piece::Knight, Color::Black);
        self.set_piece(2, 7, Piece::Bishop, Color::Black);
        self.set_piece(3, 7, Piece::Queen, Color::Black);
        self.set_piece(4, 7, Piece::King, Color::Black);
        self.set_piece(5, 7, Piece::Bishop, Color::Black);
        self.set_piece(6, 7, Piece::Knight, Color::Black);
        self.set_piece(7, 7, Piece::Rook, Color::Black);
    }

    /// A print function that prints the board to the console
    /// in a human-readable format.
    /// Will use characters like - and | to draw the board.
    pub fn print(&self, perspective: Color) {
        let columns = if perspective == Color::White {
            "    a   b   c   d   e   f   g   h"
        } else {
            "    h   g   f   e   d   c   b   a"
        };

        println!("{}", columns); 
        let rows = if perspective == Color::White {
            (0..8).rev().collect::<Vec<_>>()
        } else {
            (0..8).collect::<Vec<_>>()
        };

        for i in rows {
            println!("  +---+---+---+---+---+---+---+---+");
            let row_label = i + 1;
            print!("{} ", row_label);
            let row = &self.squares[i];
            let cols = if perspective == Color::White {
                (0..8).collect::<Vec<_>>()
            } else {
                (0..8).rev().collect::<Vec<_>>()
            };

            for j in cols {
                print!("| ");
                let square_option: &Option<Square> = &row[j];
                let symbol = match square_option {
                    Some(square) => {
                        let piece_symbol = match square.get_piece() {
                            Piece::Pawn => "P",
                            Piece::Rook => "R",
                            Piece::Knight => "N",
                            Piece::Bishop => "B",
                            Piece::Queen => "Q",
                            Piece::King => "K",
                        };
                        if *square.get_color() == Color::White {
                            piece_symbol.to_string()
                        } else {
                            piece_symbol.to_lowercase()
                        }
                    },
                    None => " ".to_string(),
                };
                print!("{} ", symbol);
            }
            println!("| {}", row_label);
        }
        println!("  +---+---+---+---+---+---+---+---+");
        println!("{}", columns); 
    }
}
