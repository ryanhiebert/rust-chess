use board::Board;
use ply::Ply;
use notation::Notation;

pub struct Game {
    pub board: Board,
    pub log: Vec<Ply>,
}

impl Game {
    pub fn new() -> Game {
        Game{ board: Board::new(), log: Vec::new() }
    }

    pub fn parse(&self, notation: &Notation, input: &str) -> Option<Ply> {
        notation.parse(&self.board, input)
    }
}


#[cfg(test)]
mod tests {
    use super::Game;
    use board::Board;
    use notation::Notation;
    use ply::{Ply, Location, Move};

    #[test]
    fn new() {
        let game = Game::new();
        assert!(game.board == Board::new());
        assert!(game.log == Vec::new());
    }

    struct TestNotation;

    impl Notation for TestNotation {
        fn parse(&self, _: &Board, _: &str)-> Option<Ply> {
            Some(Ply::Basic(Move {
                from: Location { file: 0, rank: 1 },
                to: Location { file: 0, rank: 2 },
            }, None))
        }
    }

    #[test]
    fn parse() {
        let notation = &TestNotation;
        let expected = Some(Ply::Basic(Move {
                from: Location { file: 0, rank: 1 },
                to: Location { file: 0, rank: 2 },
        }, None));
        let game = Game::new();
        assert!(game.parse(notation, "random") == expected);
    }
}
