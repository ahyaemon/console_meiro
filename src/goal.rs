use pos::Pos;

pub struct Goal {
    pos: Pos
}

impl Goal {

    pub fn new(pos: Pos) -> Goal {
        Goal{ pos }
    }

    pub fn exists(&self, pos: &Pos) -> bool {
        *pos == self.pos
    }

}


// -------------------------
// Tests
// -------------------------
#[cfg(test)]
mod tests {

    use super::*;

    // ゴールの生成
    #[test]
    fn position_test(){
        let gpos = Pos::new(2, 3);
        let g = Goal::new(gpos);
        let pos = Pos::new(2, 3);
        assert!(g.exists(&pos));
    }

}
