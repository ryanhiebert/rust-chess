#[derive(PartialEq)]
pub enum Piece {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}


#[test]
fn pieces() {
    assert!(Piece::Pawn == Piece::Pawn);
    assert!(Piece::Rook == Piece::Rook);
    assert!(Piece::Knight == Piece::Knight);
    assert!(Piece::Bishop == Piece::Bishop);
    assert!(Piece::Queen == Piece::Queen);
    assert!(Piece::King == Piece::King);
}
