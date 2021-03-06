extern crate chess;

use std::io;
use std::io::prelude::*;


fn main() {
    println!("This is Chess.");
    let input_notation = &chess::notation::StandardAlgebraicNotation;
    let output_notation = &chess::notation::DisplayBoardNotation::new(chess::notation::StandardAlgebraicNotation);

    let mut game = &mut chess::Game::new();
    println!("{}", game.unparse_board(output_notation));

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let ply = game.parse_ply(input_notation, &line.unwrap());
        match ply {
            Some(ply) => game.play(&ply),
            None      => panic!("Not a valid move."),
        }
        println!("{}", game.unparse_board(output_notation));
    }

    println!("Thanks for playing!");
}
