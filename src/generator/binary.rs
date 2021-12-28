extern crate rand;

use rand::seq::SliceRandom;
use super::super::types::cell::Cell;
use super::super::types::grid::Grid;

pub fn generate<T>(grid: &mut Grid<T>)
    where T: Cell + Clone
{
    grid.visit(|grid, cell| {
        let mut cells: Vec<T> = Vec::new();

        if cell.x() < (grid.x() - 1) {
            cells.push(grid[cell.x() + 1][cell.y()].clone());
        }

        if cell.y() < (grid.y() - 1) {
            cells.push(grid[cell.x()][cell.y() + 1].clone());
        }

        if cells.len() > 0 {
            grid.link(cell, cells.choose(&mut rand::thread_rng()).unwrap());
        }
    });
}

#[cfg(test)]
mod tests {
    use super::super::super::types::cell::*;
    use super::super::super::types::grid::Grid;
    use test::Bencher;

    #[bench]
    fn bench_generate_10x10(b: &mut Bencher) {
        b.iter(|| {
            let mut grid: Grid<BaseCell> = Grid::new(10, 10);
            super::generate(&mut grid);
        });
    }

    #[bench]
    fn bench_generate_100x100(b: &mut Bencher) {
        b.iter(|| {
            let mut grid: Grid<BaseCell> = Grid::new(100, 100);
            super::generate(&mut grid);
        });
    }
}
