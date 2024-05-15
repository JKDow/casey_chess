use crate::piece_type::PieceType;


#[derive(Debug, Clone)]
pub struct Move {
    pub from_x: usize,
    pub from_y: usize,
    pub to_x: usize,
    pub to_y: usize,
    pub piece_type: PieceType,
}


