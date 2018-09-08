use super::pos::Pos;

pub struct Player {
    pos: Pos
}

impl Player {

    pub fn new(pos: Pos) -> Player {
        Player{ pos }
    }

    pub fn up(&mut self) {
        self.pos.row -= 1;
    }

    pub fn pos_up(&self) -> (i8, i8){
        ((self.pos.row - 1) as i8, self.pos.col as i8)
    }

    pub fn right(&mut self) {
        self.pos.col += 1;
    }

    pub fn pos_right(&self) -> (i8, i8){
        (self.pos.row as i8, (self.pos.col + 1) as i8)
    }

    pub fn down(&mut self) {
        self.pos.row += 1;
    }

    pub fn pos_down(&self) -> (i8, i8){
        ((self.pos.row + 1) as i8, self.pos.col as i8)
    }

    pub fn left(&mut self) {
        self.pos.col -= 1;
    }

    pub fn pos_left(&self) -> (i8, i8){
        (self.pos.row as i8, (self.pos.col - 1) as i8)
    }

    pub fn exists(&self, pos: &Pos) -> bool {
        *pos == self.pos
    }

    pub fn pos(&self) -> &Pos {
        &self.pos
    }

}

// -------------------------
// Tests
// -------------------------
#[cfg(test)]
mod tests{

    use super::*;

    // プレイヤーの生成
    #[test]
    fn position_test(){
        let ppos = Pos::new(1, 0);
        let p = Player::new(ppos);

        let pos1 = Pos::new(1, 0);
        assert!(p.exists(&pos1));

        let pos2 = Pos::new(1, 1);
        assert!(!p.exists(&pos2));
    }

    // プレイヤーの移動
    #[test]
    fn move_test(){
        let ppos = Pos::new(1, 0);
        let mut p = Player::new(ppos);

        let pos1 = Pos::new(1, 0);
        assert!(p.exists(&pos1));

        p.right();
        let pos2 = Pos::new(1, 1);
        assert!(p.exists(&pos2));

        p.down();
        let pos3 = Pos::new(2, 1);
        assert!(p.exists(&pos3));

        p.left();
        let pos4 = Pos::new(2, 0);
        assert!(p.exists(&pos4));

        p.up();
        let pos5 = Pos::new(1, 0);
        assert!(p.exists(&pos5));
    }

}
