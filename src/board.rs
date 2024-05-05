use crate::{color::Color, piece_type::PieceType, piece::Piece};

#[derive(Debug)]
pub struct Board {
    squares: Vec<Vec<Option<Piece>>>,
    move_number: u32,
    player_turn: Color,
    white_can_castle_king: bool,
    white_can_castle_queen: bool,
    black_can_castle_king: bool,
    black_can_castle_queen: bool,
    en_passant: Option<(usize, usize)>,
    halfmove: u32,
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
        Board { 
            squares, move_number: 1, 
            player_turn: Color::White, 
            white_can_castle_king: true, 
            white_can_castle_queen: true, 
            black_can_castle_king: true,
            black_can_castle_queen: true,
            en_passant: None,
            halfmove: 0,
        }
    }

    pub fn from_fen(fen: &str) -> Option<Board> {
        let mut board = Board::new();
        let fields = fen.split_whitespace().collect::<Vec<_>>();
        if fields.len() < 6 {
            return None;
        }
        // Parse the first field 
        let mut x = 0;
        let mut y = 7;
        for c in fields[0].chars() {
            if c == '/' {
                x = 0;
                y -= 1;
            } else if c.is_digit(10) {
                x += c.to_digit(10).unwrap() as usize;
            } else {
                if let Some(piece) = Piece::from_fen(c) {
                    board.squares[y][x] = Some(piece);
                } else {
                    return None;
                }
                x += 1;
            }
        }
        // Parse the second field
        if fields[1] == "w" {
            board.player_turn = Color::White;
        } else if fields[1] == "b" {
            board.player_turn = Color::Black;
        } else {
            return None;
        }
        // Parse the third field
        board.white_can_castle_king = fields[2].contains("K");
        board.white_can_castle_queen = fields[2].contains("Q");
        board.black_can_castle_king = fields[2].contains("k");
        board.black_can_castle_queen = fields[2].contains("q");
        // Parse the fourth field
        if fields[3] == "-" {
            board.en_passant = None;
        } else {
            let x = fields[3].chars().nth(0).unwrap() as usize - 'a' as usize;
            let y = fields[3].chars().nth(1).unwrap() as usize - '1' as usize;
            board.en_passant = Some((x, y));
        }
        // Parse the fifth field
        board.halfmove = fields[4].parse().unwrap();
        // Parse the sixth field
        board.move_number = fields[5].parse().unwrap();

        Some(board)
    }

    pub fn starting_position() -> Board {
        let starting_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        Board::from_fen(starting_fen).unwrap()

    }

    pub fn set_piece(&mut self, x: usize, y: usize, piece: PieceType, color: Color) {
        self.squares[y][x] = Some(Piece::new(piece, color));
    }

    /// Move a pice from one square to another.
    /// Returns the piece that was taken, if any.
    /// This function does not check if the move is legal.
    /// It also does not update the player turn or increment the move number.
    pub fn unchecked_move_piece(&mut self, from_x: usize, from_y: usize, to_x: usize, to_y: usize) -> Option<Piece> {
        let piece = self.squares[from_y][from_x].take();
        let taken_piece = self.squares[to_y][to_x].take();
        self.squares[to_y][to_x] = piece;
        taken_piece
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
