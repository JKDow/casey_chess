use crate::{color::Color, errors::move_error::MoveError, move_type::MoveType, piece::Piece, piece_type::PieceType};

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

    fn is_square_attacked(&self, x: usize, y: usize, color: Color) -> bool {
        for i in 0..8 {
            for j in 0..8 {
                if let Some(piece) = &self.squares[i][j] {
                    if *piece.get_color() == color {
                        if piece.check_move(j, i, x, y) != MoveType::Illegal {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    pub fn move_piece(&mut self, from_x: usize, from_y: usize, to_x: usize, to_y: usize) -> Result<(), MoveError> {
        let piece_unmoved = match self.squares[from_y][from_x].as_ref() {
            Some(piece) => piece,
            None => return Err(MoveError::NoPieceOnSourceSquare),
        };
        match piece_unmoved.check_move(from_x, from_y, to_x, to_y) {
            MoveType::Illegal => return Err(MoveError::IllegalMove),
            MoveType::Pawn1 => {
                if self.squares[to_y][to_x].is_some() {
                    return Err(MoveError::IllegalMove);
                }
            },
            MoveType::Pawn2 => {
                if self.squares[to_y][to_x].is_some() || self.squares[from_y + 1][from_x].is_some() {
                    return Err(MoveError::IllegalMove);
                }
            }, 
            MoveType::PawnCapture => {
                if self.squares[to_y][to_x].is_none() {
                    return Err(MoveError::IllegalMove);
                }
            },
            MoveType::Rook => {
                // Check if there are any pieces in the way
                if from_x == to_x {
                    let start = from_y.min(to_y) + 1;
                    let end = from_y.max(to_y);
                    for i in start..end {
                        if self.squares[i][from_x].is_some() {
                            return Err(MoveError::IllegalMove);
                        }
                    }
                } else {
                    let start = from_x.min(to_x) + 1;
                    let end = from_x.max(to_x);
                    for i in start..end {
                        if self.squares[from_y][i].is_some() {
                            return Err(MoveError::IllegalMove);
                        }
                    }
                }
            },
            MoveType::Knight => {},
            MoveType::Bishop => {
                let x_diff = (to_x as i8 - from_x as i8).abs();
                let y_diff = (to_y as i8 - from_y as i8).abs();
                if x_diff != y_diff {
                    return Err(MoveError::IllegalMove);
                }
                let x_dir = (to_x as i8 - from_x as i8).signum();
                let y_dir = (to_y as i8 - from_y as i8).signum();
                let mut x = from_x as i8 + x_dir;
                let mut y = from_y as i8 + y_dir;
                while x != to_x as i8 {
                    if self.squares[y as usize][x as usize].is_some() {
                        return Err(MoveError::IllegalMove);
                    }
                    x += x_dir;
                    y += y_dir;
                }
            },
            MoveType::Queen => {
                let x_diff = (to_x as i8 - from_x as i8).abs();
                let y_diff = (to_y as i8 - from_y as i8).abs();
                if x_diff != y_diff && from_x != to_x && from_y != to_y {
                    return Err(MoveError::IllegalMove);
                }
                if x_diff == y_diff {
                    let x_dir = (to_x as i8 - from_x as i8).signum();
                    let y_dir = (to_y as i8 - from_y as i8).signum();
                    let mut x = from_x as i8 + x_dir;
                    let mut y = from_y as i8 + y_dir;
                    while x != to_x as i8 {
                        if self.squares[y as usize][x as usize].is_some() {
                            return Err(MoveError::IllegalMove);
                        }
                        x += x_dir;
                        y += y_dir;
                    }
                } else {
                    if from_x == to_x {
                        let start = from_y.min(to_y) + 1;
                        let end = from_y.max(to_y);
                        for i in start..end {
                            if self.squares[i][from_x].is_some() {
                                return Err(MoveError::IllegalMove);
                            }
                        }
                    } else {
                        let start = from_x.min(to_x) + 1;
                        let end = from_x.max(to_x);
                        for i in start..end {
                            if self.squares[from_y][i].is_some() {
                                return Err(MoveError::IllegalMove);
                            }
                        }
                    }
                }
            },
            MoveType::KingNormal => {},
            MoveType::KingCastle => {
                if from_y != to_y || (from_x as i8 - to_x as i8).abs() != 2 {
                    return Err(MoveError::IllegalMove);
                }
                if from_x < to_x {
                    // King side castle
                    if from_y != 0 && from_y != 7 {
                        return Err(MoveError::IllegalMove);
                    }
                    if from_x != 4 || (from_y == 0 && self.white_can_castle_king) || (from_y == 7 && self.black_can_castle_king) {
                        return Err(MoveError::IllegalMove);
                    }
                    if self.squares[from_y][5].is_some() || self.squares[from_y][6].is_some() {
                        return Err(MoveError::IllegalMove);
                    }
                    /*
                    if self.is_square_attacked(from_x, from_y, Color::opposite(self.player_turn)) {
                        return Err(MoveError::IllegalMove);
                    }
                    */
                } else {
                    // Queen side castle
                    if from_y != 0 && from_y != 7 {
                        return Err(MoveError::IllegalMove);
                    }
                    if from_x != 4 || (from_y == 0 && self.white_can_castle_queen) || (from_y == 7 && self.black_can_castle_queen) {
                        return Err(MoveError::IllegalMove);
                    }
                    
                    if self.squares[from_y][1].is_some() || self.squares[from_y][2].is_some() || self.squares[from_y][3].is_some() {
                        return Err(MoveError::IllegalMove);
                    }
                    /*
                    if self.is_square_attacked(from_x, from_y, Color::opposite(self.player_turn)) {
                        return Err(MoveError::IllegalMove);
                    }
                    */
                }
            },
        }
        
        Ok(())
    }
}
