use color::Color;
use piece::{Piece, Rank};
use ply::Location;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Taken(Piece),
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Castling {
    pub white_king: bool,
    pub white_queen: bool,
    pub black_king: bool,
    pub black_queen: bool,
}

/// The Board internal representation is derived from the Forsyth-Edwards
/// Notation (FEN) for recording the state of a game board. This
/// representation alone should be sufficient to resume a game, without
/// knowing the history of _how_ the pieces arrived at their destination.
///
/// http://en.wikipedia.org/wiki/Forsyth–Edwards_Notation
#[derive(PartialEq, Eq)]
pub struct Board {
    pub grid: [[Tile; 8]; 8],
    pub color: Color,
    pub castling: Castling,
    pub enpassant: Option<Location>,
    pub halfmove_clock: u32,
    pub fullmove_number: u32,

}

impl Board {
    pub fn new() -> Board {
        Board {
            // Chess boards are labelled starting at the bottom,
            // so the rows must be reversed in code.
            grid: [
                [
                    Tile::Taken(Piece{ rank: Rank::Rook, color: Color::White }),
                    Tile::Taken(Piece{ rank: Rank::Knight, color: Color::White }),
                    Tile::Taken(Piece{ rank: Rank::Bishop, color: Color::White }),
                    Tile::Taken(Piece{ rank: Rank::Queen, color: Color::White }),
                    Tile::Taken(Piece{ rank: Rank::King, color: Color::White }),
                    Tile::Taken(Piece{ rank: Rank::Bishop, color: Color::White }),
                    Tile::Taken(Piece{ rank: Rank::Knight, color: Color::White }),
                    Tile::Taken(Piece{ rank: Rank::Rook, color: Color::White }),
                ],
                [Tile::Taken(Piece{ rank: Rank::Pawn, color: Color::White }); 8],
                [Tile::Empty; 8],
                [Tile::Empty; 8],
                [Tile::Empty; 8],
                [Tile::Empty; 8],
                [Tile::Taken(Piece{ rank: Rank::Pawn, color: Color::Black }); 8],
                [
                    Tile::Taken(Piece{ rank: Rank::Rook, color: Color::Black }),
                    Tile::Taken(Piece{ rank: Rank::Knight, color: Color::Black }),
                    Tile::Taken(Piece{ rank: Rank::Bishop, color: Color::Black }),
                    Tile::Taken(Piece{ rank: Rank::Queen, color: Color::Black }),
                    Tile::Taken(Piece{ rank: Rank::King, color: Color::Black }),
                    Tile::Taken(Piece{ rank: Rank::Bishop, color: Color::Black }),
                    Tile::Taken(Piece{ rank: Rank::Knight, color: Color::Black }),
                    Tile::Taken(Piece{ rank: Rank::Rook, color: Color::Black }),
                ],
            ],
            color: Color::White,
            castling: Castling {
                white_king: true,
                white_queen: true,
                black_king: true,
                black_queen: true,
            },
            enpassant: None,
            halfmove_clock: 0,
            fullmove_number: 1,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::{Board, Tile, Castling};
    use piece::{Piece, Rank};
    use color::Color;

    #[test]
    fn new() {
        let board = Board::new();
        assert!(board.grid[0][0] == Tile::Taken(Piece{ rank: Rank::Rook, color: Color::White }));
        assert!(board.grid[7][7] == Tile::Taken(Piece{ rank: Rank::Rook, color: Color::Black }));
        assert!(board.grid[3][3] == Tile::Empty);
        assert!(board.grid[4][4] == Tile::Empty);
        assert!(board.color == Color::White);
        assert!(board.castling == Castling {
            white_king: true,
            white_queen: true,
            black_king: true,
            black_queen: true,
        });
        assert!(board.enpassant == None);
        assert!(board.halfmove_clock == 0);
        assert!(board.fullmove_number == 1);
    }
}
