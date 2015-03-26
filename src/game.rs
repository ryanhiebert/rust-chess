use board::Board;
use ply::Ply;

pub struct Game {
    pub board: Board,
    pub log: Vec<Ply>,
}

impl Game {
    pub fn new() -> Game {
        Game{ board: Board::new(), log: Vec::new() }
    }
}


#[cfg(test)]
mod tests {
    use super::Game;
    use board::Board;

    #[test]
    fn new() {
        let game = Game::new();
        assert!(game.board == Board::new());
        assert!(game.log == Vec::new());
    }
}
