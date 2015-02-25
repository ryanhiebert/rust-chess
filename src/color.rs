#[derive(PartialEq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn other(&self) -> Color {
        match *self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}


#[test]
fn white_other() {
    assert!(Color::White.other() == Color::Black);
}

#[test]
fn black_other() {
    assert!(Color::Black.other() == Color::White);
}
