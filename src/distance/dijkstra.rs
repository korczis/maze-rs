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
                format!(" {} ", d)
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

           for n in grid.neighbors_indices(f.x(), f.y()) {
                match distance_grid[n.x()][n.y()].distance {
                    Some(_d) => {},
                    _ => {
                        if distance_grid.is_linked_indices(f.x(), f.y(), n.x(), n.y()) {
                            distance_grid[n.x()][n.y()].distance = Some(f_distance + 1);
                            new_frontier.push(distance_grid[n.x()][n.y()]);
                        }
                    }
                }
            }
        }

        /*
        let len = frontier.len();
        let mut item = frontier[len - 1];

        item.distance = Some(new_distance);

        // Trunctate the frontier
        frontier.truncate(len - 1);

        // Set distance
        distance_grid[item.x()][item.y()].distance = item.distance;
        */

        // Set frontier
        frontier = new_frontier;
    }

    return distance_grid;
}
