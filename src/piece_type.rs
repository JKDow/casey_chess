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
