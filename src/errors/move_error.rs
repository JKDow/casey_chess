use thiserror::Error;



#[derive(Debug, Error)]
pub enum MoveError {
    #[error("The source square is empty")]
    NoPieceOnSourceSquare,
    #[error("Illegal move")]
    IllegalMove,
}
