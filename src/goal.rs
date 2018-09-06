pub struct Goal {
    row: u8,
    col: u8
}

impl Goal {

    pub fn new(pos: (u8, u8)) -> Goal {
        Goal{ row: pos.0, col: pos.1 }
    }

    pub fn exists(&self, irow: u8, icol: u8) -> bool {
        (irow == self.row) && (icol == self.col)
    }

}

#[test]
fn position_test(){
    let g = Goal::new((2, 3));
    assert_eq!(g.row, 2);
    assert_eq!(g.col, 3);
}
