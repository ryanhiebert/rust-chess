use ply::{Ply, Location, Move};
use board::{Board, Tile, Castling};
use piece::{Piece, Rank};
use color::Color;
use regex;


// Shortcut macro for creating a regular expression.
// Will panic! if the regular expression isn't valid.
macro_rules! regex {
    ($s:expr) => (regex::Regex::new($s).unwrap());
}


pub trait PlyInputNotation {
    fn parse_ply(&self, board: &Board, input: &str) -> Option<Ply>;
}


pub trait BoardOutputNotation {
    fn unparse_board(&self, board: &Board) -> String;
}


// pub struct StandardAlgebraicNotation;
// pub struct FigurineAlgebraicNotation;
// pub struct LongAlgebraicNotation;


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


/// Simplest input method to parse.
/// Takes pairs of zero based integers to form the move.
/// Captures are determined automatically.
/// Special moves are not allowed, including promotions.
///
/// Examples:
///
/// 00 01  // a1 a2
/// 77 76  // h8 h7
pub struct ZeroIntegersNotation;

impl PlyInputNotation for ZeroIntegersNotation {
    fn parse_ply(&self, board: &Board, input: &str) -> Option<Ply> {
        let re = regex!(r"^(\d)(\d) *(\d)(\d)");
        let captures = re.captures(input);
        match captures {
            None           => None,
            Some(captures) => {
                let from = self.parse_location(captures.at(1), captures.at(2));
                let to = self.parse_location(captures.at(3), captures.at(4));

                match (from, to) {
                    (Some(from), Some(to)) => {
                        let capture = match *board.tile_at(&to) {
                            Tile::Empty    => None,
                            Tile::Taken(_) => Some(to),
                        };
                        Some(Ply::Basic(Move { from: from, to: to }, capture))
                    },
                    _ => None,
                }
            },
        }
    }
}

impl ZeroIntegersNotation {
    fn parse_location(&self, file: Option<&str>, rank: Option<&str>) -> Option<Location> {
        match (file.unwrap_or("").parse().ok(), rank.unwrap_or("").parse().ok()) {
            (Some(file), Some(rank)) if file < 8 && rank < 8 => Some(Location { file: file, rank: rank }),
            _ => None,
        }
    }
}


#[cfg(test)]
mod tests {
    use board::Board;
    use ply::{Ply, Location, Move};
    use super::ZeroIntegersNotation;
    use super::PlyInputNotation;

    #[test]
    fn parse_ply() {
        let notation = ZeroIntegersNotation;
        let ply = notation.parse_ply(&Board::new(), "01 02");
        // Non-capturing move
        let expected = Some(Ply::Basic(Move {
            from: Location { file: 0, rank: 1 },
            to: Location { file: 0, rank: 2 },
         }, None));
        assert!(ply == expected);

        let ply = notation.parse_ply(&Board::new(), "77 76");
        // Capturing move
        let expected = Some(Ply::Basic(Move {
            from: Location { file: 7, rank: 7 },
            to: Location { file: 7, rank: 6 },
        }, Some(Location { file: 7, rank: 6 })));
        assert!(ply == expected);
    }
}


#[cfg(test)]
mod fen_tests {
    use board::Board;
    use super::BoardOutputNotation;
    use super::ForsythEdwardsNotation;

    #[test]
    fn unparse_board_initial() {
        let board = Board::new();
        let notation = ForsythEdwardsNotation;
        let fen = notation.unparse_board(&board);
        assert_eq!(&fen, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }
}
