use piece::Piece;
use board::{Tile,Board};

#[derive(Copy, PartialEq)]
pub struct Location {
    pub rank: u8,
    pub file: u8,
}

#[derive(PartialEq)]
pub struct Move {
    from: Location,
    to: Location,
}

pub trait Ply {

    /// Basic validation expected to be common to all implementations
    fn validate_move(&self, board: &Board, mv: &Move, capture: &Option<Location>) -> bool {
        // `from` must match the current board color
        let from_ok = match *board.tile_at(&mv.from) {
            Tile::Empty        => false,
            Tile::Taken(piece) => piece.color == board.color,
        };

        // `to` must either be empty or be a capture of the other color
        let to_ok = match (*board.tile_at(&mv.to), *capture) {
            (Tile::Taken(piece), Some(location)) => location == mv.to && piece.color == board.color.other(),
            (Tile::Empty,        None)           => true,
            _                                    => false,
        };

        from_ok && to_ok
    }

    fn validate(&self, board: &Board) -> bool;
}

pub struct Basic(Move, Option<Location>);
pub struct EnPassant(Move, Location);
pub struct Promotion(Move, Option<Location>, Piece);
pub struct Castling(Move, Move);

impl Ply for Basic {
    fn validate(&self, board: &Board) -> bool {
        let Basic(ref mv, ref capture) = *self;
        self.validate_move(board, mv, capture)
    }
}

impl Ply for EnPassant {
    fn validate(&self, board: &Board) -> bool {
        let EnPassant(ref mv, ref capture) = *self;
        self.validate_move(board, mv, &Some(*capture))
    }
}

impl Ply for Promotion {
    fn validate(&self, board: &Board) -> bool {
        let Promotion(ref mv, ref capture, _) = *self;
        self.validate_move(board, mv, capture)
    }
}

impl Ply for Castling {
    fn validate(&self, board: &Board) -> bool {
        let Castling(ref mv1, ref mv2) = *self;
        self.validate_move(board, mv1, &None) && self.validate_move(board, mv2, &None)
    }
}


#[cfg(test)]
mod tests {
    use piece::{Piece, Rank};
    use color::Color;
    use super::{Location, Move, Basic, EnPassant, Promotion, Castling};

    #[test]
    fn location_eq() {
        let loc = Location { rank: 4, file: 2 };
        assert!(loc == Location { rank: 4, file: 2 });
    }

    #[test]
    fn location_members() {
        let loc = Location { rank: 4, file: 2 };
        assert!(loc.rank == 4);
        assert!(loc.file == 2);
    }

    #[test]
    fn move_eq() {
        let mov = Move {
            from: Location { rank: 4, file: 2 },
            to: Location { rank: 5, file: 3 },
        };
        let mov2 = Move {
            from: Location { rank: 4, file: 2 },
            to: Location { rank: 5, file: 3 },
        };
        assert!(mov == mov2);
    }

    #[test]
    fn move_members() {
        let mov = Move {
            from: Location { rank: 4, file: 2 },
            to: Location { rank: 5, file: 3 },
        };
        assert!(mov.from == Location { rank: 4, file: 2 });
        assert!(mov.to == Location { rank: 5, file: 3});
    }

    #[test]
    fn basic() {
        let mov = Move {
            from: Location { rank: 4, file: 2 },
            to: Location { rank: 5, file: 3 },
        };
        let basic_ply = Basic(mov, None);

        let Basic(unply, _) = basic_ply;
        assert!(unply.from == Location { rank: 4, file: 2 });
        assert!(unply.to == Location { rank: 5, file: 3 });
    }

    #[test]
    fn enpassant() {
        let mov = Move {
            from: Location { rank: 4, file: 2 },
            to: Location { rank: 5, file: 3 },
        };
        let enpassant_ply = EnPassant(mov, Location { rank: 6, file: 2 });

        let EnPassant(unply, loc) = enpassant_ply;
        assert!(unply.from == Location { rank: 4, file: 2 });
        assert!(unply.to == Location { rank: 5, file: 3 });
        assert!(loc == Location { rank: 6, file: 2});
    }

    #[test]
    fn promotion() {
        let mov = Move {
            from: Location { rank: 4, file: 2 },
            to: Location { rank: 5, file: 3 },
        };
        let promotion_ply = Promotion(
            mov, None, Piece { rank: Rank::Knight, color: Color::White });

        let Promotion(unply, _, piece) = promotion_ply;
        assert!(unply.from == Location { rank: 4, file: 2 });
        assert!(unply.to == Location { rank: 5, file: 3 });
        assert!(piece == Piece { rank: Rank::Knight, color: Color::White });
        assert!(piece.rank == Rank::Knight);
        assert!(piece.color == Color::White);
    }

    #[test]
    fn castling() {
        let mov = Move {
            from: Location { rank: 4, file: 2 },
            to: Location { rank: 5, file: 3 },
        };
        let mov2 = Move {
            from: Location { rank: 2, file: 3 },
            to: Location { rank: 4, file: 5 },
        };
        let castling_ply = Castling(mov, mov2);

        let Castling(unply, unply2) = castling_ply;
        assert!(unply.from == Location { rank: 4, file: 2 });
        assert!(unply.to == Location { rank: 5, file: 3 });
        assert!(unply2.from == Location { rank: 2, file: 3 });
        assert!(unply2.to == Location { rank: 4, file: 5 });
    }
}
