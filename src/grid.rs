use crate::step::Step;

#[derive(Debug)]
pub struct Grid {
     grid: Vec<Vec<Cell>>,
     size: u8
}

impl Grid {
    pub fn new(size: u8) -> Self {
        let mut grid = Vec::new();
        for i in 0..size.pow(2) {
            let mut column = Vec::new();
            for j in 0..size.pow(2) {
                column.push(
                    Cell { row: i, column: j, subgrid: (i / size, j / size), potential_values: (1..=size.pow(2)).collect() }
                )
            }
            grid.push(column);
        }

        Self {
            grid, size
        }
    }

    pub fn is_solved(&self) -> bool {
        let solved = self.grid.iter().flatten().all(|cell| cell.get_value().is_some());
        println!("Is Solved: {}", solved);
        solved
    }

    pub fn add_data(&mut self, step: &Step) {
        // let row = &mut self.grid[step.row as usize];
        // let cell = &mut row[step.column as usize];
        // cell.set_value(step.value);

        self.grid[step.row as usize][step.column as usize].remove_value(step.value);

        for mut cell in self.grid.into_iter().flatten().filter(|cell| cell.row == cell.row).into_iter() {
            cell.remove_value(step.value);
        }

        // for mut cell in &self.grid.iter().flatten().filter(|cell| cell.column == cell_being_updated.column) {
        //     cell.remove_value(step.value);
        // }
        //
        // for mut cell in &self.grid.iter().flatten().filter(|cell| cell.subgrid == cell_being_updated.subgrid) {
        //     cell.remove_value(step.value);
        // }
            ()
    }
}

#[derive(Debug)]
pub struct Cell {
    pub row: u8,
    pub column: u8,
    pub subgrid: (u8,u8),
    pub potential_values: Vec<u8>,
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
