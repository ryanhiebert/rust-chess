use ply::{Ply, Location, Move};
use board::{Board, Tile};
use color::Color;
use piece::{Piece, Rank};
use regex;

use notation::{PlyInputNotation, BoardOutputNotationHelper};


/// Standard Algebraic Notation
/// Takes the standard algebraic notation of the positions of the move.
/// Captures are determined automatically.
/// Both positions must be written full-form.
///
/// Examples:
///
/// a1 a2
/// h8 h7
pub struct StandardAlgebraicNotation;

impl BoardOutputNotationHelper for StandardAlgebraicNotation {
    fn unparse_tile(&self, tile: &Tile) -> String {
        match *tile {
            Tile::Empty            => " ".to_string(),
            Tile::Taken(ref piece) => self.unparse_piece(piece).to_string(),
        }
    }

    fn unparse_location(&self, location: &Location) -> String {
        let mut output = self.file_label(location.file);
        output.push_str(&self.rank_label(location.rank));
        output
    }

    fn file_label(&self, file: u8) -> String {
        let label = match file {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            7 => 'h',
            _ => panic!("Invalid board file."),
        };
        label.to_string()
    }

    fn rank_label(&self, rank: u8) -> String {
        (rank + 1).to_string()
    }
}

impl PlyInputNotation for StandardAlgebraicNotation {
    fn parse_ply(&self, board: &Board, input: &str) -> Option<Ply> {
        let re = regex!(r"^([a-h])([1-8]) *([a-h])([1-8])");
        let captures = re.captures(input);
        match captures {
            None           => None,
            Some(captures) => {
                let from = self.parse_location(captures.at(1), captures.at(2));
                let to = self.parse_location(captures.at(3), captures.at(4));

                match (from, to) {
                    (Some(from), Some(to)) => {
                        let capture = match *board.tile_at(&to) {
                            Tile::Empty    => None,
                            Tile::Taken(_) => Some(to),
                        };
                        Some(Ply::Basic(Move { from: from, to: to }, capture))
                    },
                    _ => None,
                }
            },
        }
    }
}

impl StandardAlgebraicNotation {
    fn parse_location(&self, file: Option<&str>, rank: Option<&str>) -> Option<Location> {
        let file = match file.unwrap_or("") {
            "a" => Some(0),
            "b" => Some(1),
            "c" => Some(2),
            "d" => Some(3),
            "e" => Some(4),
            "f" => Some(5),
            "g" => Some(6),
            "h" => Some(7),
            _   => None,
        };
        let rank: Option<u8> = rank.unwrap_or("").parse::<u8>().ok();
        match (file, rank) {
            (Some(file), Some(rank)) if (0 < rank && rank < 9) => {
                Some(Location { file: file, rank: rank - 1u8})
            },
            _ => None,
        }
    }

    fn unparse_piece(&self, piece: &Piece) -> char {
        let chr = match piece.rank {
            Rank::Pawn   => 'p',
            Rank::Rook   => 'r',
            Rank::Knight => 'n',
            Rank::Bishop => 'b',
            Rank::Queen  => 'q',
            Rank::King   => 'k',
        };

        match piece.color {
            Color::White => chr.to_uppercase().next().unwrap(),
            Color::Black => chr.to_lowercase().next().unwrap(),
        }
    }
}


#[cfg(test)]
mod tests {
    use board::Board;
    use ply::{Ply, Location, Move};
    use super::StandardAlgebraicNotation;
    use notation::PlyInputNotation;

    #[test]
    fn parse_ply_nocapture() {
        let notation = StandardAlgebraicNotation;
        let ply = notation.parse_ply(&Board::new(), "a2 a3");

        let expected = Some(Ply::Basic(Move {
            from: Location { file: 0, rank: 1 },
            to: Location { file: 0, rank: 2 },
        }, None));
        assert_eq!(ply, expected);
    }

    #[test]
    fn parse_ply_capture() {
        let notation = StandardAlgebraicNotation;
        let ply = notation.parse_ply(&Board::new(), "h8 h7");

        let expected = Some(Ply::Basic(Move {
            from: Location { file: 7, rank: 7 },
            to: Location { file: 7, rank: 6 },
        }, Some(Location { file: 7, rank: 6 })));
        assert_eq!(ply, expected);
    }
}
