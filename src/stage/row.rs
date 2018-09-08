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

    pub fn cells(&self) -> &Vec<u8> {
        &self.cells
    }

    pub fn cell(&self, index: usize) -> &u8 {
        &self.cells[index]
    }

    pub fn count_cells(&self) -> u8 {
        self.cells.len() as u8
    }

}
