extern crate rand;

use rand::Rng;

use super::super::types::cell::Cell;
use super::super::types::grid::Grid;

pub fn generate<T>(grid: &mut Grid<T>)
    where T: Cell + Clone
{
    for y in 0..grid.y() {
        let mut cells: Vec<T> = Vec::new();
        for x in 0..grid.x() {
            cells.push(grid.cells[x][y].clone());

            let at_eastern_boundary = x == grid.x() - 1;
            let at_northern_boundary = y == grid.y() - 1;

            let should_close_out = at_eastern_boundary || (!at_northern_boundary && rand::thread_rng().gen());

            let mut should_clear = false;
            if should_close_out {
                let member = rand::thread_rng().choose(&cells).unwrap();
                if y < (grid.y() - 1) {
                    grid.link_indices(member.x(), member.y(), member.x(), member.y() + 1);
                }
                should_clear = true;
            } else {
                grid.link_indices(x, y, x + 1, y);
            }

            if should_clear {
                cells.clear();
            }
        }
    }
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