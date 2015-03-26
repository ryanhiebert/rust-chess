use color::Color;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Rank {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Piece {
    pub rank: Rank,
    pub color: Color,
}


#[cfg(test)]
mod tests {
    use super::{Rank, Piece};
    use color::Color;

    #[test]
    fn ranks() {
        assert!(Rank::Pawn == Rank::Pawn);
        assert!(Rank::Rook == Rank::Rook);
        assert!(Rank::Knight == Rank::Knight);
        assert!(Rank::Bishop == Rank::Bishop);
        assert!(Rank::Queen == Rank::Queen);
        assert!(Rank::King == Rank::King);
    }

    #[test]
    fn pieces() {
        let white = Piece { rank: Rank::Queen, color: Color::White };
        let black = Piece { rank: Rank::King, color: Color::Black };

        assert!(white == Piece { rank: Rank::Queen, color: Color::White });
        assert!(black == Piece { rank: Rank::King, color: Color::Black });

        assert!(white.rank == Rank::Queen);
        assert!(white.color == Color::White);
        assert!(black.rank == Rank::King);
        assert!(black.color == Color::Black);
    }
}
