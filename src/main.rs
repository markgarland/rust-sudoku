
mod grid;
use grid::Grid;


fn main() {

    let mut grid = Grid::new(2);
    const GRID_DATA: [Option<u8>; 16] = [Some(1), Some(2), Some(3), Some(4), Some(3), Some(4), None, None, Some(2), None, None, None, Some(4), None, None, None];
    grid.load_data(GRID_DATA);
    // println! ("{:?}", grid)
    println! ("{:?}", grid.get_columns())
}



