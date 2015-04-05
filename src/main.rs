extern crate chess;


#[cfg(not(test))]
fn main() {
    println!("This is Chess.");
    let notation = &chess::notation::ZeroIntegersNotation;
    let mut game = &mut chess::Game::new();
    let ply = game.parse_ply(notation, "01 02");

    match ply {
        Some(ply) => game.play(&ply),
        None      => panic!("Not a valid move."),
    };
}
