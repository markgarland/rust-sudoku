
mod grid;
mod step;

use grid::Grid;
use step::Step;


fn main() {

    let mut grid = Grid::new(2);

    let mut steps = Vec::new();
    steps.push(Step {row:0,column:0,value:1});
    steps.push(Step {row:0,column:1,value:2});
    steps.push(Step {row:0,column:2,value:3});
    steps.push(Step {row:0,column:3,value:4});
    steps.push(Step {row:1,column:0,value:3});
    steps.push(Step {row:1,column:1,value:4});
    steps.push(Step {row:2,column:0,value:2});
    steps.push(Step {row:3,column:0,value:4});

    let mut steps_iterator = steps.iter().peekable();
    while !grid.is_solved() && steps_iterator.peek().is_some() {
        grid.add_data(steps_iterator.next().unwrap());
    }


    println! ("{}", grid.is_solved());

    // dbg! (grid);
}

