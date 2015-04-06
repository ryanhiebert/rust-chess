use ply::Ply;
use board::Board;

pub use notation::fen::ForsythEdwardsNotation;
pub use notation::zero_int::ZeroIntegersNotation;


// Shortcut macro for creating a regular expression.
// Will panic! if the regular expression isn't valid.
macro_rules! regex {
    ($s:expr) => (regex::Regex::new($s).unwrap());
}


pub trait PlyInputNotation {
    fn parse_ply(&self, board: &Board, input: &str) -> Option<Ply>;
}


pub trait BoardOutputNotation {
    fn unparse_board(&self, board: &Board) -> String;
}


mod fen;
mod zero_int;
