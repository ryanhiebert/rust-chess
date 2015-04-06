use ply::Location;
use board::{Board, Tile, Castling};
use piece::{Piece, Rank};
use color::Color;

use notation::BoardOutputNotation;


/// Forsyth-Edwards Notation (FEN).
/// Describes the current state of a board.
pub struct ForsythEdwardsNotation;

impl BoardOutputNotation for ForsythEdwardsNotation {
    fn unparse_board(&self, board: &Board) -> String {
        format!(
            "{} {} {} {} {} {}",
            self.unparse_grid(&board.grid),
            self.unparse_color(&board.color),
            self.unparse_castling(&board.castling),
            self.unparse_enpassant(&board.enpassant),
            board.halfmove_clock,
            board.fullmove_number,
        )
    }
}

impl ForsythEdwardsNotation {
    fn unparse_grid(&self, grid: &[[Tile; 8]; 8]) -> String {
        // Rows are in reverse order: from the top to the bottom.
        format!(
            "{}/{}/{}/{}/{}/{}/{}/{}",
            self.unparse_rank(&grid[7]),
            self.unparse_rank(&grid[6]),
            self.unparse_rank(&grid[5]),
            self.unparse_rank(&grid[4]),
            self.unparse_rank(&grid[3]),
            self.unparse_rank(&grid[2]),
            self.unparse_rank(&grid[1]),
            self.unparse_rank(&grid[0]),
        )
    }

    fn unparse_rank(&self, rank: &[Tile; 8]) -> String {
        let mut output = String::new();

        let mut empty_counter = 0;
        for tile in rank.iter() {
            match *tile {
                Tile::Empty            => { empty_counter += 1; },
                Tile::Taken(ref piece) => {
                    if empty_counter > 0 {
                        output.push_str(&empty_counter.to_string());
                        empty_counter = 0;
                    }
                    output.push(self.unparse_piece(piece));
                },
            }
        }

        if empty_counter > 0 {
            output.push_str(&empty_counter.to_string());
        }

        output
    }

    fn unparse_piece(&self, piece: &Piece) -> char {
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
            Color::Black => chr,
        }
    }

    fn unparse_color(&self, color: &Color) -> &str {
        match *color {
            Color::White => "w",
            Color::Black => "b",
        }
    }

    fn unparse_castling(&self, castling: &Castling) -> String {
        let mut output = String::new();

        if castling.white_king { output.push('K'); };
        if castling.white_queen { output.push('Q'); };
        if castling.black_king { output.push('k'); };
        if castling.black_queen { output.push('q'); };

        if &output == "" { output.push('-'); };

        output
    }

    fn unparse_enpassant(&self, enpassant: &Option<Location>) -> String {
        match *enpassant {
            None           => "-".to_string(),
            Some(location) => {
                let file = match location.file {
                    0 => "A",
                    1 => "B",
                    2 => "C",
                    3 => "D",
                    4 => "E",
                    5 => "F",
                    6 => "G",
                    7 => "H",
                    _ => panic!("Internal En passant file invalid."),
                };
                let rank = if location.rank < 8 {
                    location.rank.to_string()
                } else {
                    panic!("Internal En passant rank invalid.")
                };
                file.to_string() + &rank
            },
        }
    }
}


#[cfg(test)]
mod tests {
    use board::Board;
    use notation::BoardOutputNotation;
    use super::ForsythEdwardsNotation;

    #[test]
    fn unparse_board_initial() {
        let board = Board::new();
        let notation = ForsythEdwardsNotation;
        let fen = notation.unparse_board(&board);
        assert_eq!(&fen, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }
}
