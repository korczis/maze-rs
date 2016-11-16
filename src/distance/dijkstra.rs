use super::super::types::cell::Cell;
use super::super::types::grid::Grid;

const EMPTY_CELL: &'static str = "   ";

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DistanceCell {
    x: usize,
    y: usize,
    distance: Option<usize>,
}

impl Cell for DistanceCell {
    fn new(x: usize, y: usize) -> DistanceCell {
        DistanceCell {
            x: x,
            y: y,
            distance: None
        }
    }

    fn to_string(&self) -> String {
        match self.distance {
            Some(d) => {
                format!(" {} ", d % 10)
            },
            _ => EMPTY_CELL.to_string()
        }
    }

    fn x(&self) -> usize {
        self.x
    }

    fn y(&self) -> usize {
        self.y
    }
}

impl DistanceCell {
    fn distance(&self) -> Option<usize> {
        self.distance
    }
}

pub fn calculate<T>(grid: &Grid<T>) -> Grid<DistanceCell>
    where T: Cell + Clone
{
    let mut distance_grid: Grid<DistanceCell> = Grid::new(grid.x(), grid.y());

    distance_grid.links = grid.links.clone();

    let mut frontier = Vec::new();
    distance_grid[0][0].distance = Some(0);
    frontier.push(distance_grid[0][0]);

    while frontier.len() > 0 {
        // Crete new frontiers
        let mut new_frontier = Vec::new();

        for f in frontier {
            let f_distance = f.distance.unwrap();

           for neighbor in grid.neighbors_indices(f.x(), f.y()) {
                match distance_grid[neighbor.x()][neighbor.y()].distance() {
                    Some(_d) => {},
                    _ => {
                        if distance_grid.is_linked_indices(f.x(), f.y(), neighbor.x(), neighbor.y()) {
                            distance_grid[neighbor.x()][neighbor.y()].distance = Some(f_distance + 1);
                            new_frontier.push(distance_grid[neighbor.x()][neighbor.y()]);
                        }
                    }
                }
            }
        }

        // Set frontier
        frontier = new_frontier;
    }

    return distance_grid;
}

#[cfg(test)]
mod tests {
    use super::super::super::types::cell::*;
    use super::super::super::distance;
    use super::super::super::types::grid::Grid;
    use test::Bencher;

    #[bench]
    fn bench_calculate_10x10(b: &mut Bencher) {
        b.iter(|| {
            let mut grid: Grid<BaseCell> = Grid::new(10, 10);
            grid.generate_aldous_broder();
            let _ = distance::dijkstra::calculate(&grid);
        });
    }

    #[bench]
    fn bench_calculate_100x100(b: &mut Bencher) {
        b.iter(|| {
            let mut grid: Grid<BaseCell> = Grid::new(100, 100);
            grid.generate_aldous_broder();
            let _ = distance::dijkstra::calculate(&grid);
        });
    }
}
