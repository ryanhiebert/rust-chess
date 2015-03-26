#![feature(plugin)]
#![plugin(regex_macros)]
extern crate regex;

pub use color::Color;
pub use piece::{Piece, Rank};
pub use ply::{Location, Move, Ply};
pub use board::{Board, Tile};
pub use game::Game;
pub use parser::{Notation, FromToZeroIntegers};

mod color;
mod piece;
mod ply;
mod board;
mod game;
mod parser;
