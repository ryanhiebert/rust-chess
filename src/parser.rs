use ply::{Ply, Location, Move};
use board::Board;


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
    fn parse(&self, _: &Board, _: &str) -> Option<Ply> {
        //let re = regex!(r"^(\d{2}) *(\d{2})");
        Some(Ply::Basic(Move {
            from: Location { file: 0, rank: 0 },
            to: Location { file: 0, rank: 1 },
        }, None))
    }
}


#[cfg(tests)]
mod tests {
    use board::Board;
    use super::FromToZeroIntegers;

    #[test]
    fn parse() {
        let notation = FromToZeroIntegers;
        let ply = notation.parse(&Board::new(), "00 01");
        let expected = Option(Ply::Basic(Move {
            from: Location { file: 0, rank: 0 },
            to: Location { file: 0, rank: 1 },
         }, None));
        assert!(ply == expected);
    }
}
