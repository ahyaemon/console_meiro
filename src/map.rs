use row::Row;

pub struct Map {
    rows: Vec<Row>
}

impl Map {
    pub fn new(text: &str) -> Map {
        let mut rows = vec![];
        for line in text.trim().split('\n') {
            let row = Row::from_line(line);
            rows.push(row);
        };
        Map{ rows }
    }

    pub fn to_text(&self) -> String {
        self.rows.iter()
        .map(|row| row.to_line())
        .collect::<Vec<String>>()
        .join("\n")
    }

    pub fn rows(&self) -> Vec<&Vec<u8>> {
        let mut rows = vec![];
        for row in &self.rows {
            rows.push(row.cells());
        }
        rows
    }
}