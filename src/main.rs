mod cell;

use std::any::type_name_of_val;
use cell::Cell;


fn main() {
    const CELL_WIDTH:u8 = 3;
    const CELL_HEIGHT:u8 = 3;

    const CELLS_WIDTH:u8 = 3;
    const CELLS_HEIGHT:u8 = 3;

    const NUMBER_OF_CELLS:u8 = CELL_WIDTH*CELLS_WIDTH*CELL_HEIGHT*CELLS_HEIGHT;

    // let grid = [Cell::new(None); (CELL_WIDTH*CELLS_WIDTH*CELL_HEIGHT*CELLS_HEIGHT) as usize];
    // println! ("{:?}", grid)

    // let _array: [Cell; 100] = [const {Cell::new(None)}; 100];

    // const FOO: Cell = Cell { value: None };
    // let _array: [Cell; 100] =
    // let foo_array: [Cell; 100] = [const {Cell {value: None}}; 100];

    // let grid: [Option<u8>; NUMBER_OF_CELLS as usize] = [None; NUMBER_OF_CELLS as usize];


    let mut grid = Vec::new();
    for i in 0..NUMBER_OF_CELLS {
        grid.push (
            Cell {grid_index: i+1, value: None}
        )
    }
    // println! ("{:?}", grid);

    let rows = grid.chunks((CELL_WIDTH * CELL_HEIGHT) as usize);
    rows.map(|x|x.to_vec()).collect());
    // 
    // rows.map(|c| c.to_vec()).collect();
    // println! ("{:?}", rows);


    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let v_chunked: Vec<Vec<i32>> = v.chunks(3).map(|x| x.to_vec()).collect();
    println!("{:?}", v_chunked);
}



