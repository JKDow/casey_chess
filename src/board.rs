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
    /// Creates a new empty board.
    /// # Description
    /// The board is represented as a 2D array of Option<Piece>.
    /// Each square can either contain a piece or be empty.
    /// # Inputs/Outputs
    /// - Inputs: None
    /// - Returns: An empty board.
    /// # Example
    /// ``` Rust
    /// let board = Board::new();
    /// ```
    pub fn new() -> Self {
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

    /// Parse a FEN string and create a board.
    /// # Description
    /// FEN (Forsyth-Edwards Notation) is a standard notation for describing a particular board position of a chess game.
    /// The FEN string contains 6 fields separated by spaces:
    /// 1. Piece placement Each piece is identified by a single letter (uppercase for white, lowercase for black) and empty squares are represented by a number.
    /// 2. Active color. "w" means white moves next, "b" means black moves next.
    /// 3. Castling availability. Each letter indicates whether castling is possible for each side (KQkq) or not (-).
    /// 4. En passant target square in algebraic notation. If there is no en passant target square, this is "-".
    /// 5. Halfmove clock: The number of halfmoves since the last capture or pawn advance. This is used to determine if a draw can be claimed under the fifty-move rule.
    /// 6. Fullmove number: The number of the full move. It starts at 1, and is incremented after black moves.
    /// # Inputs/Outputs
    /// - Inputs: A FEN string.
    /// - Returns: A board if the FEN string is valid, otherwise None.
    /// # Example
    /// ``` Rust
    /// let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    /// // this will print the starting position of a chess game to the console
    /// board.print(Color::White);
    /// ```
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

    /// Get the starting position of a chess game.
    /// # Description
    /// Uses a FEN string to create a board with the starting position of a chess game.
    /// # Inputs/Outputs
    /// - Inputs: None
    /// - Returns: A board with the starting position of a chess game.
    /// # Example
    /// ``` Rust
    /// let board = Board::starting_position();
    /// // this will print the starting position of a chess game to the console
    /// board.print(Color::White);
    /// ```
    pub fn starting_position() -> Board {
        let starting_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        Board::from_fen(starting_fen).unwrap()

    }

    /// Print the board to the console.
    /// # Description
    /// Prints the board to the console with the given perspective.
    /// Also contains the coordinates of the board.
    /// # Inputs/Outputs
    /// - Inputs: The perspective of the board.
    /// - Returns: Nothing.
    /// # Example
    /// ``` Rust
    /// let board = Board::starting_position();
    /// board.print(Color::White);
    /// ```
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

    /// Get the first piece in a given direction.
    /// # Description
    /// Returns the coordinates of the first piece in the given direction.
    /// If there is no piece in the given direction, it returns None.
    /// dx and dy are the direction in which to look for a piece.
    /// These values should be -1, 0, or 1.
    /// # Inputs/Outputs
    /// - Input: The x and y coordinate of the start
    /// - Input: The direction in which to look for a piece
    /// - Returns: The coordinates of the first piece in the given direction, if any.
    /// # Example
    /// ``` Rust
    /// let board = Board::starting_position();
    /// // Starting from e4 and going north, the first piece is on e7
    /// let (x, y) = board.first_piece_in_direction(4, 3, 0, 1).unwrap();
    /// assert_eq!(x, 4);
    /// assert_eq!(y, 6);
    /// ```
    fn first_piece_in_direction(&self, x: usize, y: usize, dx: i8, dy: i8) -> Option<(usize, usize)> {
        let mut x = x as i8 + dx;
        let mut y = y as i8 + dy;
        while x >= 0 && x < 8 && y >= 0 && y < 8 {
            if self.squares[y as usize][x as usize].is_some() {
                return Some((x as usize, y as usize));
            }
            x += dx;
            y += dy;
        }
        None
    }

    /// Check if a square is attacked by a piece of a given color.
    /// # Description
    /// Checks if a square is attacked by a piece of a given color.
    /// This function is used to check if a king is in check.
    /// Also used to check if a square is attacked for castling
    /// # Inputs/Outputs
    /// - Input: The x and y coordinate of the square
    /// - Input: The color of the attacking pieces
    /// - Returns: True if the square is attacked, otherwise false.
    /// # Example
    /// ``` Rust
    /// let board = Board::starting_position();
    /// // The square e3 is attacked by a white pawn
    /// assert!(board.is_square_attacked(4, 2, Color::White));
    /// ```
    fn is_square_attacked(&self, x: usize, y: usize, color: Color) -> bool {
        // Define static arrays that get used internally to the function
        static LINE_PIECES: [PieceType; 2] = [PieceType::Rook, PieceType::Queen];
        static DIAGONAL_PIECES: [PieceType; 2] = [PieceType::Bishop, PieceType::Queen];
        static DIAGONAL_PIECES_WITH_PAWN: [PieceType; 3] = [PieceType::Bishop, PieceType::Queen, PieceType::Pawn];
        static KNIGHT: [PieceType; 1] = [PieceType::Knight];
        static STRAIGHT_DIRECTIONS: [(i8, i8); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        static DIAGONAL_DIRECTIONS: [(i8, i8); 4] = [(1, 1), (-1, 1), (1, -1), (-1, -1)];
        // Helper function to check if a piece is of the given color and type
        let is_piece = |piece: Option<&Piece>, check: &[PieceType]| -> bool {
            piece.map_or(false, |p| *p.get_color() == color && check.contains(p.get_type()))
        };
        // look for rooks and queens
        for (dx, dy) in &STRAIGHT_DIRECTIONS {
            if let Some((x, y)) = self.first_piece_in_direction(x, y, *dx, *dy) {
                if is_piece(self.squares[y][x].as_ref(), &LINE_PIECES) { return true; }
            }
        }
        // look for bishops, queens and pawns
        for (dx, dy) in &DIAGONAL_DIRECTIONS {
            if let Some((x, y)) = self.first_piece_in_direction(x, y, *dx, *dy) {
                if (color == Color::White && *dy == -1) || (color == Color::Black && *dy == 1){
                    if is_piece(self.squares[y][x].as_ref(), &DIAGONAL_PIECES_WITH_PAWN) {
                        return true;
                    }
                } else {
                    if is_piece(self.squares[y][x].as_ref(), &DIAGONAL_PIECES) {
                        return true;
                    }
                }
            }
        }
        // look for knights
        let knight_moves = [(1, 2), (2, 1), (-1, 2), (-2, 1), (1, -2), (2, -1), (-1, -2), (-2, -1)];
        knight_moves.iter().any(|&(dx, dy)| {
            let (nx, ny) = (x as i8 + dx, y as i8 + dy);
            (0..8).contains(&nx) && (0..8).contains(&ny) && is_piece(self.squares[ny as usize][nx as usize].as_ref(), &KNIGHT)
        })
    }

    /// Move a piece from one square to another.
    /// This function needs refactoring to be more readable.
    /// Move code for each piece into its own function.
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
                if self.squares[to_y][to_x].is_none() && self.en_passant != Some((to_x, to_y)) {
                    // Check for en passant
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
            MoveType::KingCastleKingSide => {
                if self.player_turn == Color::White {
                    if !self.white_can_castle_king {
                        return Err(MoveError::IllegalMove);
                    }
                    if self.is_square_attacked(4, 0, Color::White) || self.is_square_attacked(5, 0, Color::White) || self.is_square_attacked(6, 0, Color::White) {
                        return Err(MoveError::IllegalMove);
                    }
                    if self.squares[0][5].is_some() || self.squares[0][6].is_some() {
                        return Err(MoveError::IllegalMove);
                    }
                } else {
                    if !self.black_can_castle_king {
                        return Err(MoveError::IllegalMove);
                    }
                    if self.is_square_attacked(4, 7, Color::Black) || self.is_square_attacked(5, 7, Color::Black) || self.is_square_attacked(6, 7, Color::Black) {
                        return Err(MoveError::IllegalMove);
                    }
                    if self.squares[7][5].is_some() || self.squares[7][6].is_some() {
                        return Err(MoveError::IllegalMove);
                    }
                }
            },
            MoveType::KingCastleQueenSide => {
                if self.player_turn == Color::White {
                    if !self.white_can_castle_queen {
                        return Err(MoveError::IllegalMove);
                    }
                    if self.is_square_attacked(4, 0, Color::White) || self.is_square_attacked(3, 0, Color::White) || self.is_square_attacked(2, 0, Color::White) {
                        return Err(MoveError::IllegalMove);
                    }
                    if self.squares[0][3].is_some() || self.squares[0][2].is_some() || self.squares[0][1].is_some() {
                        return Err(MoveError::IllegalMove);
                    }
                } else {
                    if !self.black_can_castle_queen {
                        return Err(MoveError::IllegalMove);
                    }
                    if self.is_square_attacked(4, 7, Color::Black) || self.is_square_attacked(3, 7, Color::Black) || self.is_square_attacked(2, 7, Color::Black) {
                        return Err(MoveError::IllegalMove);
                    }
                    if self.squares[7][3].is_some() || self.squares[7][2].is_some() || self.squares[7][1].is_some() {
                        return Err(MoveError::IllegalMove);
                    }
                }
            },
        }
        
        Ok(())
    }
}
