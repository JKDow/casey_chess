use std::fmt::{self, Display, Formatter};


#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

impl PieceType {
    pub fn to_centipawns(&self) -> i32 {
        match self {
            PieceType::Pawn => 100,
            PieceType::Rook => 500,
            PieceType::Knight => 300,
            PieceType::Bishop => 300,
            PieceType::Queen => 900,
            PieceType::King => 0,
        }
    }
}

impl Display for PieceType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            PieceType::Pawn => "P",
            PieceType::Rook => "R",
            PieceType::Knight => "N",
            PieceType::Bishop => "B",
            PieceType::Queen => "Q",
            PieceType::King => "K",
        })
    }
}

impl TryFrom<char> for PieceType {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'P' => Ok(PieceType::Pawn),
            'R' => Ok(PieceType::Rook),
            'N' => Ok(PieceType::Knight),
            'B' => Ok(PieceType::Bishop),
            'Q' => Ok(PieceType::Queen),
            'K' => Ok(PieceType::King),
            _ => Err(()),
        }
    }
}
