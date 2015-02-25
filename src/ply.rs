use piece::Piece;

#[derive(PartialEq)]
pub struct Location {
    pub rank: u8,
    pub file: u8,
}

#[derive(PartialEq)]
pub struct Move {
    from: Location,
    to: Location,
}

#[derive(PartialEq)]
pub enum Ply {
    Basic(Move),
    EnPassant(Move),
    Promotion(Move, Piece),
    Castling(Move, Move),
}


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
fn ply_basic() {
    let mov = Move {
        from: Location { rank: 4, file: 2 },
        to: Location { rank: 5, file: 3 },
    };
    let basic_ply = Ply::Basic(mov);

    match basic_ply {
        Ply::Basic(unply) => {
            assert!(unply.from == Location { rank: 4, file: 2 });
            assert!(unply.to == Location { rank: 5, file: 3 });
        },
        _ => panic!("Not a basic ply."),
    }
}

#[test]
fn ply_enpassant() {
    let mov = Move {
        from: Location { rank: 4, file: 2 },
        to: Location { rank: 5, file: 3 },
    };
    let enpassant_ply = Ply::EnPassant(mov);

    match enpassant_ply {
        Ply::EnPassant(unply) => {
            assert!(unply.from == Location { rank: 4, file: 2 });
            assert!(unply.to == Location { rank: 5, file: 3 });
        },
        _ => panic!("Not an enpassant ply."),
    }
}

#[test]
fn ply_promotion() {
    let mov = Move {
        from: Location { rank: 4, file: 2 },
        to: Location { rank: 5, file: 3 },
    };
    let promotion_ply = Ply::Promotion(mov, Piece::Knight);

    match promotion_ply {
        Ply::Promotion(unply, piece) => {
            assert!(unply.from == Location { rank: 4, file: 2 });
            assert!(unply.to == Location { rank: 5, file: 3 });
            assert!(piece == Piece::Knight);
        },
        _ => panic!("Not a promotion ply."),
    }
}

#[test]
fn ply_castling() {
    let mov = Move {
        from: Location { rank: 4, file: 2 },
        to: Location { rank: 5, file: 3 },
    };
    let mov2 = Move {
        from: Location { rank: 2, file: 3 },
        to: Location { rank: 4, file: 5 },
    };
    let castling_ply = Ply::Castling(mov, mov2);

    match castling_ply {
        Ply::Castling(unply, unply2) => {
            assert!(unply.from == Location { rank: 4, file: 2 });
            assert!(unply.to == Location { rank: 5, file: 3 });
            assert!(unply2.from == Location { rank: 2, file: 3 });
            assert!(unply2.to == Location { rank: 4, file: 5 });
        },
        _ => panic!("Not a castling ply."),
    }
}
