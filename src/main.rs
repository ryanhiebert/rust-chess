extern crate chess;


#[cfg(not(test))]
fn main() {
    println!("This is Chess.");
    let input_notation = &chess::notation::ZeroIntegersNotation;
    let output_notation = &chess::notation::DisplayBoardNotation::new(chess::notation::ZeroIntegersNotation);

    let mut game = &mut chess::Game::new();
    println!("{}", game.unparse_board(output_notation));

    let ply = game.parse_ply(input_notation, "01 02");
    match ply {
        Some(ply) => game.play(&ply),
        None      => panic!("Not a valid move."),
    };
    println!("{}", game.unparse_board(output_notation));
}
