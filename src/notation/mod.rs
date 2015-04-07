use ply::Ply;
use board::{Board, Tile};

pub use notation::fen::ForsythEdwardsNotation;
pub use notation::zero_int::ZeroIntegersNotation;
pub use notation::displayboard::DisplayBoardNotation;


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

pub trait BoardOutputNotationHelper {
    fn unparse_tile(&self, tile: &Tile) -> String;
    fn file_label(&self, file: u8) -> String;
    fn rank_label(&self, rank: u8) -> String;
}


mod fen;
mod zero_int;
mod displayboard;
