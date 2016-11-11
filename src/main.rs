extern crate maze;

use maze::*;

fn main() {
    let mut grid = Grid::new(3, 3);
    println!("{:?}", grid);

    let ref mut cell = grid[0][0];
    println!("{:?}", cell);
}
