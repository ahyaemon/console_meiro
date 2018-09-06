pub struct Player {
    row: u8,
    col: u8
}

impl Player {

    pub fn new(text: &str) -> Player {
        for (irow, row) in text.trim().split("\n").enumerate() {
            for (icol, col) in row.chars().enumerate() {
                if col == 'P' {
                    return Player{ row: irow as u8, col: icol as u8 }
                }
            }
        }
        Player{ row: 0, col: 0 }
    }

    pub fn up(&self) {

    }

    pub fn exists(&self, irow: u8, icol: u8) -> bool {
        (irow == self.row) && (icol == self.col)
    }

}

#[test]
fn position_test(){
    let map = test_map();
    let p = Player::new(&map);
    assert_eq!(p.row, 1);
    assert_eq!(p.col, 0);
}

#[test]
fn move_test(){
    let map = test_map();
    let p = Player::new(&map);
    p.up();
    assert_eq!(p.row, 1);
    assert_eq!(p.col, 0);
}

fn test_map() -> String {
r#"
1111
P011
100G
1111
"#.to_string()
}