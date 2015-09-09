use board::{Board, Castling};
use ply::Location;
use color::Color;
use notation::{BoardOutputNotation, BoardOutputNotationHelper};


pub struct DisplayBoardNotation<T: BoardOutputNotationHelper> {
    helper: T,
}

impl<T: BoardOutputNotationHelper> BoardOutputNotation for DisplayBoardNotation<T> {
    fn unparse_board(&self, board: &Board) -> String {
        let mut output = String::new();

        // The header
        output.push_str(" ");
        for file in 0..8u8 {
            output.push_str("   ");
            output.push_str(&self.helper.file_label(file));
        }
        output.push_str("\n");

        // The grid
        let divider = "  +---+---+---+---+---+---+---+---+\n";
        output.push_str(divider);

        for rank in (0..8u8).rev() {
            output.push_str(&self.helper.rank_label(rank));
            output.push_str(" |");
            for file in 0..8u8 {
                output.push_str(" ");
                output.push_str(&self.helper.unparse_tile(board.tile_at(
                    &Location { file: file, rank: rank }
                )));
                output.push_str(" |");
            }
            output.push_str("\n");
            output.push_str(divider);
        }

        // The footer, with supporting information
        output.push_str(&format!(
            "            {} {} {} {} {}\n",
            match board.color { Color::White => "w", Color::Black => "b" },
            self.unparse_castling(&board.castling),
            self.unparse_enpassant(&board.enpassant),
            board.halfmove_clock,
            board.fullmove_number,
        ));

        output
    }
}

impl<T: BoardOutputNotationHelper> DisplayBoardNotation<T> {
    pub fn new(helper: T) -> DisplayBoardNotation<T> {
        DisplayBoardNotation::<T> { helper: helper }
    }

    fn unparse_castling(&self, castling: &Castling) -> String {
        let mut output = String::new();

        output.push_str(if castling.white_king { "K" } else { "" });
        output.push_str(if castling.white_queen { "Q" } else { "" });
        output.push_str(if castling.black_king { "k" } else { "" });
        output.push_str(if castling.black_queen { "q" } else { "" });

        if &output == "" { output.push_str("-") };
        output
    }

    fn unparse_enpassant(&self, enpassant: &Option<Location>) -> String {
        match *enpassant {
            Some(location) => self.helper.unparse_location(&location),
            None           => "-".to_string()
        }
    }
}


#[cfg(test)]
mod tests {
    use notation::{BoardOutputNotation, BoardOutputNotationHelper};
    use ply::Location;
    use color::Color;
    use board::{Board, Tile};
    use piece::Rank;
    use super::DisplayBoardNotation;

    pub struct TestHelper;

    impl BoardOutputNotationHelper for TestHelper {
        fn unparse_tile(&self, tile: &Tile) -> String {
            match *tile {
                Tile::Empty        => " ".to_string(),
                Tile::Taken(piece) => {
                    let chr = match piece.rank {
                        Rank::Pawn   => 'p',
                        Rank::Rook   => 'r',
                        Rank::Knight => 'n',
                        Rank::Bishop => 'b',
                        Rank::Queen  => 'q',
                        Rank::King   => 'k',
                    };

                    match piece.color {
                        Color::White => chr.to_uppercase().next().unwrap(),
                        Color::Black => chr
                    }.to_string()
                },
            }
        }

        fn unparse_location(&self, location: &Location) -> String { "-".to_string() }

        fn file_label(&self, file: u8) -> String { file.to_string() }
        fn rank_label(&self, rank: u8) -> String { rank.to_string() }
    }


    #[test]
    fn unparse_board_new() {
        let notation = DisplayBoardNotation { helper: TestHelper };
        let expected = "    0   1   2   3   4   5   6   7\n  \
                          +---+---+---+---+---+---+---+---+\n\
                        7 | r | n | b | q | k | b | n | r |\n  \
                          +---+---+---+---+---+---+---+---+\n\
                        6 | p | p | p | p | p | p | p | p |\n  \
                          +---+---+---+---+---+---+---+---+\n\
                        5 |   |   |   |   |   |   |   |   |\n  \
                          +---+---+---+---+---+---+---+---+\n\
                        4 |   |   |   |   |   |   |   |   |\n  \
                          +---+---+---+---+---+---+---+---+\n\
                        3 |   |   |   |   |   |   |   |   |\n  \
                          +---+---+---+---+---+---+---+---+\n\
                        2 |   |   |   |   |   |   |   |   |\n  \
                          +---+---+---+---+---+---+---+---+\n\
                        1 | P | P | P | P | P | P | P | P |\n  \
                          +---+---+---+---+---+---+---+---+\n\
                        0 | R | N | B | Q | K | B | N | R |\n  \
                          +---+---+---+---+---+---+---+---+\n            \
                                    w KQkq - 0 1\n";
        let board = Board::new();
        assert_eq!(notation.unparse_board(&board), expected);
    }
}
