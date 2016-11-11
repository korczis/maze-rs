use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Cell {
    pub x: u64,
    pub y: u64
}

#[derive(Debug)]
pub struct Grid {
    pub cells: Vec<Vec<Cell>>
}

impl Grid {
    pub fn new(x: u64, y: u64) -> Grid {
        let mut grid = Grid {
            cells: Vec::new()
        };

        // let mut cells: Vec<Vec<Cell>> = Vec::new();
        for i in 0..x {
            let mut row = Vec::new();
            for j in 0..y {
                row.push(Cell {
                    x: i,
                    y: j
                });

            }
            grid.cells.push(row);
        }

        return grid;
    }
}

impl Index<usize> for Grid {
    type Output = Vec<Cell>;

    fn index<'a>(&'a self, index: usize) -> &'a Vec<Cell> {
        &self.cells[index]
    }
}

impl IndexMut<usize> for Grid {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut Vec<Cell> {
        &mut self.cells[index]
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
