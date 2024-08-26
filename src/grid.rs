
#[derive(Debug)]
pub struct Grid {
     grid: Vec<Cell>,
     size: u8
}

impl Grid {
    pub fn new(size: u8) -> Self {
        let mut grid = Vec::new();
        for i in 0..size.pow(4) {
            grid.push (
                Cell {grid_index: i+1, potential_values: (1..=size.pow(2)).collect()}
            )
        }

        Self {
            grid, size
        }
    }

    pub fn load_data(&mut self, data: [Option<u8>; 16]) {
        for i in 0..data.len() {
            match data[i] {
                Some(v) => self.grid[i].set_value(v),
                _ => {}
            }
        }
    }

    pub fn get_rows(&self) -> Vec<Vec<&Cell>> {
        self.grid.chunks(self.size.pow(2) as usize).map(|x| x.into_iter().collect()).collect()
    }

    pub fn get_columns(&self) -> Vec<Vec<&Cell>> {
        let mut result: Vec<Vec<&Cell>> = Vec::new();
        for i in 1..=self.size.pow(2) {
            let mut result_item : Vec<&Cell> = Vec::new();
            for j in (i as usize..=self.grid.len() as usize).step_by(self.size.pow(2) as usize) {
                result_item.push(&self.grid[j-1]);
            }
            result.push(result_item);
        }
        result
    }

}

#[derive(Debug)]
pub struct Cell {
    pub grid_index: u8,
    pub potential_values: Vec<u8>
}

impl Cell {
    pub fn get_value(&self) -> Option<u8> {
        match self.potential_values.len() {
            1 => Some(self.potential_values[0]),
            _ => None
        }
    }

    pub fn set_value(&mut self, value: u8) {
        self.potential_values.clear();
        self.potential_values.push(value);
    }

    pub fn remove_value(&mut self, value: u8) {
        self.potential_values.remove(self.potential_values.iter().position(|&p| p == value).unwrap() as usize);
    }
}
