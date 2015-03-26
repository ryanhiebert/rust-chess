use ply::Ply;
use board::Board;


pub trait Notation {
    fn parse(&self, board: &Board, input: &str) -> Option<Ply>;
}


pub struct StandardAlgebraicNotation;
pub struct FigurineAlgebraicNotation;
pub struct LongAlgebraicNotation;


/// Simple algebraic notation for noting from and to moves.
/// Only basic moves supported.
pub struct FromToAlgebraicNotation;
