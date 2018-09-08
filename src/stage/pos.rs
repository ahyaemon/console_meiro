#[derive(Debug, PartialEq)]
pub struct Pos {
    pub row: u8,
    pub col: u8
}

impl Pos {

    pub fn new(row: u8, col: u8) -> Pos {
        Pos{ row, col }
    }

}