use board::Board;

pub struct Game {
    pub board: Board
}

impl Game {
    pub fn new() -> Game {
        Game{ board: Board::new() }
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
    }
}
