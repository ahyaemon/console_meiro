pub struct Text (String);

impl Text {

    pub fn from_str(s: String) -> Text {
        Text(s.trim().to_string())
    }

    /// テキストの中から`c`の位置を探し、`(row, col)`形式のタプルを返す
    pub fn find_position(&self, c: char) -> (u8, u8) {
        for (irow, row) in self.0.split("\n").enumerate() {
            for (icol, col) in row.chars().enumerate() {
                if col == c {
                    return (irow as u8, icol as u8)
                }
            }
        }
        (0, 0)
    }

    /// 保持するStringの参照を返す
    pub fn borrow_str(&self) -> &str {
        &self.0
    }

}