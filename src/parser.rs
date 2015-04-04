use ply::{Ply, Location, Move};
use board::{Board, Tile};
use regex;


// Shortcut macro for creating a regular expression.
// Will panic! if the regular expression isn't valid.
macro_rules! regex {
    ($s:expr) => (regex::Regex::new($s).unwrap());
}


pub trait Notation {
    fn parse(&self, board: &Board, input: &str) -> Option<Ply>;
}


// pub struct StandardAlgebraicNotation;
// pub struct FigurineAlgebraicNotation;
// pub struct LongAlgebraicNotation;


/// Simplest input method to parse.
/// Takes pairs of zero based integers to form the move.
/// Captures are determined automatically.
/// Special moves are not allowed, including promotions.
///
/// Examples:
///
/// 00 01  // a1 a2
/// 77 76  // h8 h7
pub struct ZeroIntegersNotation;

impl Notation for ZeroIntegersNotation {
    fn parse(&self, board: &Board, input: &str) -> Option<Ply> {
        let re = regex!(r"^(\d)(\d) *(\d)(\d)");
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

impl ZeroIntegersNotation {
    fn parse_location(&self, file: Option<&str>, rank: Option<&str>) -> Option<Location> {
        match (file.unwrap_or("").parse().ok(), rank.unwrap_or("").parse().ok()) {
            (Some(file), Some(rank)) if file < 8 && rank < 8 => Some(Location { file: file, rank: rank }),
            _ => None,
        }
    }
}


#[cfg(test)]
mod tests {
    use board::Board;
    use ply::{Ply, Location, Move};
    use super::ZeroIntegersNotation;
    use super::Notation;

    #[test]
    fn parse() {
        let notation = ZeroIntegersNotation;
        let ply = notation.parse(&Board::new(), "01 02");
        // Non-capturing move
        let expected = Some(Ply::Basic(Move {
            from: Location { file: 0, rank: 1 },
            to: Location { file: 0, rank: 2 },
         }, None));
        assert!(ply == expected);

        let ply = notation.parse(&Board::new(), "77 76");
        // Capturing move
        let expected = Some(Ply::Basic(Move {
            from: Location { file: 7, rank: 7 },
            to: Location { file: 7, rank: 6 },
        }, Some(Location { file: 7, rank: 6 })));
        assert!(ply == expected);
    }
}
