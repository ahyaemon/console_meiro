use pos::Pos;

pub struct Text (String);

impl Text {

    pub fn from_str(s: String) -> Text {
        Text(s.trim().to_string())
    }

    /// テキストの中から`c`の位置を探し、`(row, col)`形式のタプルを返す
    pub fn find_position(&self, c: char) -> Pos {
        for (irow, row) in self.0.split("\n").enumerate() {
            for (icol, col) in row.chars().enumerate() {
                if col == c {
                    return Pos::new(irow as u8, icol as u8 )
                }
            }
        }
        Pos::new(0, 0)
    }

    /// 保持するStringの参照を返す
    pub fn borrow_str(&self) -> &str {
        &self.0
    }

}

#[cfg(test)]
mod tests{

    use super::*;

    // 'P' の位置を探す
    #[test]
    fn search_p(){
        let map_str = utils::map_str();
        let text = Text::from_str(map_str);
        let ppos = Pos::new(1, 0);
        assert_eq!(text.find_position('P'), ppos);
    }

    // trimされているか？
    #[test]
    fn trim(){
        let map_str = utils::map_str();
        let text = Text::from_str(map_str);
        let str_actual = utils::map_str_trimmed();
        assert_eq!(text.borrow_str(), &str_actual);
    }

    mod utils{
        pub fn map_str() -> String {
            r#"
1111
P011
100G
1111
            "#.to_string()
        }

        pub fn map_str_trimmed() -> String {
            r#"1111
P011
100G
1111"#.to_string()            
        }
    }

}