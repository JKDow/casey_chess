use thiserror::Error;



#[derive(Debug, Error)]
pub enum MoveError {
    #[error("The source square is empty")]
    NoPieceOnSourceSquare,
    #[error("Must move piece")]
    MustMovePiece,
    #[error("Illegal move")]
    IllegalMove,
    #[error("King is in check")]
    KingInCheck,
    #[error("Cannot capture own piece")]
    CannotCaptureOwnPiece,
    #[error("Cannot move opponent's piece")]
    PieceWrongColor,
    #[error("Move is blocked")] 
    MoveBlocked,    
}
