use super::row::Row;

pub struct Map {
    rows: Vec<Row>,
    nrow: u8,
    ncol: u8
}

impl Map {

    pub fn new(map_str: &str) -> Map {
        let mut rows = vec![];
        for line in map_str.trim().split('\n') {
            let row = Row::from_line(line);
            rows.push(row);
        };

        let nrow = rows.len() as u8;
        let ncol = rows[0].count_cells();

        Map{ rows, nrow, ncol }
    }

    pub fn rows(&self) -> Vec<&Vec<u8>> {
        let mut rows = vec![];
        for row in &self.rows {
            rows.push(row.cells());
        }
        rows
    }

    pub fn out_of_range(&self, pos: &(i8, i8)) -> bool {
        if pos.0 < 0 {
            true
        }
        else if pos.0 >= self.nrow as i8 {
            true
        }
        else if pos.1 < 0 {
            true
        }
        else if pos.1 >= self.ncol as i8 {
            true
        }
        else{
            false
        }
    }

    pub fn empty_space(&self, pos: &(u8, u8)) -> bool {
        let row = &self.rows[pos.0 as usize];
        let cell = row.cell(pos.1 as usize);
        *cell == 0u8
    }

}


#[cfg(test)]
mod tests {

    use super::*;

    // mapからはみ出した判定のテスト
    #[test]
    fn test_out_of_range(){
        let map_str = utils::map_str_trimmed();
        let map = Map::new(&map_str);
        
        assert!(!map.out_of_range(&(0, 0)));
        assert!(map.out_of_range(&(-1, 0)));
        assert!(map.out_of_range(&(4, 0)));
        assert!(map.out_of_range(&(0, -1)));
        assert!(map.out_of_range(&(0, 4)));

    }

    // 移動可能なセルかのテスト
    #[test]
    fn test_empty_space(){
        let map_str = utils::map_str_trimmed();
        let map = Map::new(&map_str);

        assert!(map.empty_space(&(1, 0)));
        assert!(map.empty_space(&(1, 1)));
        assert!(map.empty_space(&(2, 3)));
        assert!(!map.empty_space(&(0, 0)));
        assert!(!map.empty_space(&(3, 3)));

    }

    mod utils {

        // nrow: 4, ncol: 4
        pub fn map_str_trimmed() -> String {
            r#"1111
P011
100G
1111"#.to_string()            
        }

    }

}