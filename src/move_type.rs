
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MoveType {
    Illegal,
    Pawn1,
    Pawn2,
    PawnCapture,
    Rook,
    Knight,
    Bishop,
    Queen,
    KingNormal,
    KingCastleKingSide,
    KingCastleQueenSide,
}
