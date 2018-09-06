pub struct Player {
    row: u8,
    col: u8
}

impl Player {

    pub fn new(pos: (u8, u8)) -> Player {
        Player{ row: pos.0, col: pos.1 }
    }

    pub fn up(&mut self) {
        self.row -= 1;
    }

    pub fn right(&mut self) {
        self.col += 1;
    }

    pub fn down(&mut self) {
        self.row += 1;
    }

    pub fn left(&mut self) {
        self. col -= 1;
    }

    pub fn exists(&self, irow: u8, icol: u8) -> bool {
        (irow == self.row) && (icol == self.col)
    }

}

#[test]
fn position_test(){
    let p = Player::new((1, 0));
    assert!(p.exists(1, 0));
    assert!(!p.exists(1, 1));
}

#[test]
fn move_test(){
    let mut p = Player::new((1, 0));
    assert!(p.exists(1, 0));
    p.right();
    assert!(p.exists(1, 1));
    p.down();
    assert!(p.exists(2, 1));
    p.left();
    assert!(p.exists(2, 0));
    p.up();
    assert!(p.exists(1, 0));
}
