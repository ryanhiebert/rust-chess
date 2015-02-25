use color::Color;
use piece::{Piece, Rank};
use ply::Ply;

#[derive(PartialEq)]
pub enum Tile {
    Empty,
    Taken(Piece),
}

pub struct Board {
    pub grid: [[Tile; 8]; 8],
    pub moves: Vec<Ply>,
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
                [
                    Tile::Taken(Piece{ rank: Rank::Pawn, color: Color::White }),
                    Tile::Taken(Piece{ rank: Rank::Pawn, color: Color::White }),
                    Tile::Taken(Piece{ rank: Rank::Pawn, color: Color::White }),
                    Tile::Taken(Piece{ rank: Rank::Pawn, color: Color::White }),
                    Tile::Taken(Piece{ rank: Rank::Pawn, color: Color::White }),
                    Tile::Taken(Piece{ rank: Rank::Pawn, color: Color::White }),
                    Tile::Taken(Piece{ rank: Rank::Pawn, color: Color::White }),
                    Tile::Taken(Piece{ rank: Rank::Pawn, color: Color::White }),
                ],
                [
                    Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty,
                    Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty,
                ],
                [
                    Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty,
                    Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty,
                ],
                [
                    Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty,
                    Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty,
                ],
                [
                    Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty,
                    Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty,
                ],
                [
                    Tile::Taken(Piece{ rank: Rank::Pawn, color: Color::Black }),
                    Tile::Taken(Piece{ rank: Rank::Pawn, color: Color::Black }),
                    Tile::Taken(Piece{ rank: Rank::Pawn, color: Color::Black }),
                    Tile::Taken(Piece{ rank: Rank::Pawn, color: Color::Black }),
                    Tile::Taken(Piece{ rank: Rank::Pawn, color: Color::Black }),
                    Tile::Taken(Piece{ rank: Rank::Pawn, color: Color::Black }),
                    Tile::Taken(Piece{ rank: Rank::Pawn, color: Color::Black }),
                    Tile::Taken(Piece{ rank: Rank::Pawn, color: Color::Black }),
                ],
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
            ],
            moves: vec![],
        }
    }
}


#[cfg(test)]
mod tests {
    use super::{Board, Tile};
    use piece::{Piece, Rank};
    use color::Color;

    #[test]
    fn new() {
        let board = Board::new();
        assert!(board.grid[0][0] == Tile::Taken(Piece{ rank: Rank::Rook, color: Color::White }));
        assert!(board.grid[7][7] == Tile::Taken(Piece{ rank: Rank::Rook, color: Color::White }));
        assert!(board.grid[3][3] == Tile::Empty);
        assert!(board.grid[4][4] == Tile::Empty);
        assert!(board.moves == vec![]);
    }
}
