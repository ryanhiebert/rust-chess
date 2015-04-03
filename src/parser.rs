use ply::{Ply, Location, Move};
use board::Board;
use regex;


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
pub struct FromToZeroIntegers;

impl Notation for FromToZeroIntegers {
    fn parse(&self, _: &Board, input: &str) -> Option<Ply> {
        let re = regex!(r"^(\d)(\d) *(\d)(\d)");
        let captures = re.captures(input);
        match captures {
            None           => None,
            Some(captures) => {
                let from = self.parse_location(captures.at(1), captures.at(2));
                let to = self.parse_location(captures.at(3), captures.at(4));

                match (from, to) {
                    (Some(from), Some(to)) => {
                        Some(Ply::Basic(Move { from: from, to: to }, None))
                    },
                    _ => None,
                }
            },
        }
    }
}

impl FromToZeroIntegers {
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
    use super::FromToZeroIntegers;
    use super::Notation;

    #[test]
    fn parse() {
        let notation = FromToZeroIntegers;
        let ply = notation.parse(&Board::new(), "00 01");
        let expected = Some(Ply::Basic(Move {
            from: Location { file: 0, rank: 0 },
            to: Location { file: 0, rank: 1 },
         }, None));
        assert!(ply == expected);

        let ply = notation.parse(&Board::new(), "77 76");
        let expected = Some(Ply::Basic(Move {
            from: Location { file: 7, rank: 7 },
            to: Location { file: 7, rank: 6 },
        }, None));
        assert!(ply == expected);
    }
}
