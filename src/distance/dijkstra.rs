use super::super::types::cell::Cell;
use super::super::types::grid::Grid;

const EMPTY_CELL: &'static str = "   ";

static ASCII_LOWER: [char; 62] = [
'0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't',
'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D',
'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N',
'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X',
'Y', 'Z'
];

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DistanceCell {
    x: usize,
    y: usize,
    distance: Option<usize>,
    is_path: bool,
    // path: Vec<(usize, usize)>
}

impl Cell for DistanceCell {
    fn new(x: usize, y: usize) -> DistanceCell {
        DistanceCell {
            x: x,
            y: y,
            distance: None,
            is_path: false,
            // path: Vec::new()
        }
    }

    fn to_string(&self) -> String {
        match self.distance {
            Some(d) => {
                if self.is_path {
                    format!(" {} ", ASCII_LOWER[d % 62])
                } else {
                    EMPTY_CELL.to_string()
                }
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
    pub fn distance(&self) -> Option<usize> {
        self.distance
    }
}

pub fn calculate<T>(grid: &Grid<T>, begin: (usize, usize), end: (usize, usize)) -> Grid<DistanceCell>
    where T: Cell + Clone + Copy
{
    let mut distance_grid: Grid<DistanceCell> = Grid::new(grid.x(), grid.y());

    distance_grid.links = grid.links.clone();

    let mut frontier = Vec::new();
    distance_grid[begin.0][begin.1].distance = Some(0);
    frontier.push(distance_grid[begin.0][begin.1]);

    while frontier.len() > 0 {
        // Crete new frontiers
        let mut new_frontier = Vec::new();

        for f in frontier {
            let f_distance = f.distance.unwrap();

           for neighbor in grid.neighbors_linked_indices(f.x(), f.y()) {
                match distance_grid[neighbor.x()][neighbor.y()].distance() {
                    Some(_d) => {},
                    _ => {
                        distance_grid[neighbor.x()][neighbor.y()].distance = Some(f_distance + 1);
                        new_frontier.push(distance_grid[neighbor.x()][neighbor.y()]);
                    }
                }
            }
        }

        // Set frontier
        frontier = new_frontier;
    }

    let mut current = Some(distance_grid[end.0][end.1]);
    distance_grid[end.0][end.1].is_path = true;

    while current.is_some() {
        let c = current.unwrap();
        let mut new = None;
        for n in distance_grid.neighbors_linked_indices(c.x(), c.y()) {
            if n.distance.unwrap() < c.distance.unwrap() {
                new = Some(n);
            }
        }

        if new.is_some() {
            let n = new.unwrap();
            distance_grid[n.x()][n.y()].is_path = true;
        }

        current = new;
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
