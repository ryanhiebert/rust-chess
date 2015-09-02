use color::Color;
use piece::{Piece, Rank};
use ply::Location;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Tile {
    Empty,
    Taken(Piece),
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
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
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
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
                    Tile::Taken(Piece { color: Color::White, rank: Rank::Rook }),
                    Tile::Taken(Piece { color: Color::White, rank: Rank::Knight }),
                    Tile::Taken(Piece { color: Color::White, rank: Rank::Bishop }),
                    Tile::Taken(Piece { color: Color::White, rank: Rank::Queen }),
                    Tile::Taken(Piece { color: Color::White, rank: Rank::King }),
                    Tile::Taken(Piece { color: Color::White, rank: Rank::Bishop }),
                    Tile::Taken(Piece { color: Color::White, rank: Rank::Knight }),
                    Tile::Taken(Piece { color: Color::White, rank: Rank::Rook }),
                ],
                [Tile::Taken(Piece { color: Color::White, rank: Rank::Pawn }); 8],
                [Tile::Empty; 8],
                [Tile::Empty; 8],
                [Tile::Empty; 8],
                [Tile::Empty; 8],
                [Tile::Taken(Piece { color: Color::Black, rank: Rank::Pawn }); 8],
                [
                    Tile::Taken(Piece { color: Color::Black, rank: Rank::Rook }),
                    Tile::Taken(Piece { color: Color::Black, rank: Rank::Knight }),
                    Tile::Taken(Piece { color: Color::Black, rank: Rank::Bishop }),
                    Tile::Taken(Piece { color: Color::Black, rank: Rank::Queen }),
                    Tile::Taken(Piece { color: Color::Black, rank: Rank::King }),
                    Tile::Taken(Piece { color: Color::Black, rank: Rank::Bishop }),
                    Tile::Taken(Piece { color: Color::Black, rank: Rank::Knight }),
                    Tile::Taken(Piece { color: Color::Black, rank: Rank::Rook }),
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

    pub fn tile_at(&self, location: &Location) -> &Tile {
        &self.grid[location.rank as usize][location.file as usize]
    }
}


#[cfg(test)]
mod tests {
    use super::{Board, Tile, Castling};
    use piece::{Piece, Rank};
    use color::Color;
    use ply::Location;

    #[test]
    fn new() {
        let board = Board::new();
        assert!(board.grid[0][0] == Tile::Taken(Piece { rank: Rank::Rook, color: Color::White }));
        assert!(board.grid[7][7] == Tile::Taken(Piece { rank: Rank::Rook, color: Color::Black }));
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

    #[test]
    fn tile_at() {
        let location = Location { rank: 0, file: 0 };
        let expected_piece = Piece { rank: Rank::Rook, color: Color::White };
        assert!(Board::new().tile_at(&location) == &Tile::Taken(expected_piece));
    }

    #[test]
    fn clone_and_mutate() {
        let board1 = Board::new();
        let mut board2 = board1.clone();
        board2.color = Color::Black;
        assert!(board1.color == Color::White);
        assert!(board2.color == Color::Black);
    }
}
