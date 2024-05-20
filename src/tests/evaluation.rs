use crate::board::Board;


#[test]
fn basic_evaluation_0() {
    let board = Board::starting_position();
    assert_eq!(board.basic_evaluate(), 0);
}

#[test]
fn basic_evaluation_1() {
    let board = Board::from_fen("r1bk2nr/ppp3pp/2n1Pp2/6B1/2B5/5N2/P1P2PPP/bN3RK1 w - - 0 10").unwrap();
    assert_eq!(board.basic_evaluate(), -500);
}

#[test]
fn basic_evaluation_2() {
    let board = Board::from_fen("4K1B1/2q2p2/2rr4/2pk4/n7/P6P/4P1b1/4B1qQ w - - 0 1").unwrap();
    assert_eq!(board.basic_evaluate(), -1800);
}

#[test]
fn basic_evaluation_3() {
    let board = Board::from_fen("1b6/2k2N2/p1P5/1P1K4/5p2/3p4/p2R2B1/8 w - - 0 1").unwrap();
    assert_eq!(board.basic_evaluate(), 600);
}


