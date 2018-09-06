pub struct Goal {
    row: u8,
    col: u8
}

impl Goal {

    pub fn new(text: &str) -> Goal {
        for (irow, row) in text.trim().split("\n").enumerate() {
            for (icol, col) in row.chars().enumerate() {
                if col == 'G' {
                    return Goal{ row: irow as u8, col: icol as u8 }
                }
            }
        }
        Goal{ row: 0, col: 0 }
    }

    pub fn exists(&self, irow: u8, icol: u8) -> bool {
        (irow == self.row) && (icol == self.col)
    }

}

#[test]
fn position_test(){
    let text = r#"
1111
P011
100G
1111
    "#;
    let g = Goal::new(text);
    assert_eq!(g.row, 2);
    assert_eq!(g.col, 3);
}
