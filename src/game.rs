use board::{Board, Tile};
use ply::{Ply, Location, Move};
use notation::{PlyInputNotation, BoardOutputNotation};

pub struct Game {
    pub board: Board,
    pub log: Vec<Ply>,
}

impl Game {
    pub fn new() -> Game {
        Game{ board: Board::new(), log: Vec::new() }
    }

    pub fn parse_ply(&self, notation: &PlyInputNotation, input: &str) -> Option<Ply> {
        notation.parse_ply(&self.board, input)
    }

    pub fn unparse_board(&self, notation: &BoardOutputNotation) -> String {
        notation.unparse_board(&self.board)
    }

    fn play_basic(&mut self, mv: Move, capture: Option<Location>) {
        match capture {
            Some(location) => {
                self.board.grid[location.rank as usize][location.file as usize] = Tile::Empty;
            },
            None => (),
        };

        // Move the piece to the new tile
        // TODO: The move needs to be validated.
        self.board.grid[mv.to.rank as usize][mv.to.file as usize] = self.board.grid[mv.from.rank as usize][mv.from.file as usize];
        self.board.grid[mv.from.rank as usize][mv.from.file as usize] = Tile::Empty;
    }

    pub fn play(&mut self, ply: &Ply) {
        match *ply {
            Ply::Basic(mv, capture) => self.play_basic(mv, capture),
            _ => panic!("I don't know what to do with that ply yet."),
        }

        // If we didn't need to panic, then store the ply in the log.
        self.log.push(*ply);

        self.board.color = self.board.color.other();
    }
}


#[cfg(test)]
mod tests {
    use super::Game;
    use board::Board;
    use notation::PlyInputNotation;
    use ply::{Ply, Location, Move};

    #[test]
    fn new() {
        let game = Game::new();
        assert!(game.board == Board::new());
        assert!(game.log == Vec::new());
    }

    struct TestPlyInputNotation;

    impl PlyInputNotation for TestPlyInputNotation {
        fn parse_ply(&self, _: &Board, _: &str)-> Option<Ply> {
            Some(Ply::Basic(Move {
                from: Location { file: 0, rank: 1 },
                to: Location { file: 0, rank: 2 },
            }, None))
        }
    }

    #[test]
    fn parse_ply() {
        let notation = &TestPlyInputNotation;
        let expected = Some(Ply::Basic(Move {
                from: Location { file: 0, rank: 1 },
                to: Location { file: 0, rank: 2 },
        }, None));
        let game = Game::new();
        assert!(game.parse_ply(notation, "random") == expected);
    }
}
