extern crate rand;

use rand::seq::SliceRandom;
use super::super::types::cell::Cell;
use super::super::types::grid::Grid;

pub fn generate<T>(grid: &mut Grid<T>)
    where T: Cell + Clone
{
    let mut cell = grid.random_cell();
    let mut unvisited = grid.size() - 1;

    while unvisited > 0 {
        let neighbors = grid.neighbors(&cell);
        let neighbor = neighbors.choose(&mut rand::thread_rng()).unwrap();

        if !grid.links.contains_key(&(neighbor.x(), neighbor.y())) {
            grid.link(&cell, &neighbor);
            unvisited -= 1;
        }

        cell = neighbor.clone();
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