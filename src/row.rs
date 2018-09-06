pub struct Row{
    cells: Vec<u8>
}

impl Row {
    pub fn from_line(line: &str) -> Row {
        let mut cells = vec![];
        for cell in line.chars() {
            match cell {
                '0' => cells.push(0),
                '1' => cells.push(1),
                _ => cells.push(0)
            };
        };

        Row{ cells }
    }

    pub fn to_line(&self) -> String {
        self.cells.iter()
                .map(|cell| cell.to_string())
                .collect::<Vec<String>>()
                .concat()
    }

    pub fn cells(&self) -> &Vec<u8> {
        return &self.cells;
    }
}
