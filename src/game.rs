use board::{Board, Tile};
use ply::{Ply, Location, Move};
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

    fn play_basic(&mut self, mv: Move, capture: Option<Location>) {
        match capture {
            Some(location) => {
                self.board.grid[location.file as usize][location.rank as usize] = Tile::Empty;
            },
            None => (),
        };

        // Move the piece to the new tile
        // TODO: The move needs to be validated.
        self.board.grid[mv.to.file as usize][mv.to.rank as usize] = self.board.grid[mv.from.file as usize][mv.from.rank as usize];
        self.board.grid[mv.from.file as usize][mv.from.rank as usize] = Tile::Empty;

    }

    pub fn play(&mut self, ply: &Ply) {
        match *ply {
            Ply::Basic(mv, capture) => self.play_basic(mv, capture),
            _ => panic!("I don't know what to do with that ply yet."),
        }

        // If we didn't need to panic, then store the ply in the log.
        self.log.push(*ply);
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
