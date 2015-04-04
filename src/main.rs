extern crate chess;


#[cfg(not(test))]
fn main() {
    let notation = &chess::notation::ZeroIntegersNotation;
    let game = &chess::Game::new();
    game.parse(notation, "01 02");
    println!("This is chess!");
}
