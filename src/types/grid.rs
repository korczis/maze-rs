extern crate rand;

use std::collections::BTreeSet;
use std::collections::HashMap;
use std::ops::{Index, IndexMut};

use rand::distributions::{IndependentSample, Range};

use super::cell::Cell;
use super::super::generator;
use super::super::output;

#[derive(Debug, Serialize, Deserialize)]
pub struct Grid<T>
    where T: Cell
{
    x: usize,
    y: usize,
    pub cells: Vec<Vec<T>>,
    pub links: HashMap<(usize, usize), BTreeSet<(usize, usize)>>
}

impl <T> Grid<T>
    where T: Cell + Clone
{
    pub fn new(x: usize, y: usize) -> Grid<T> {
        let mut grid = Grid {
            x: x,
            y: y,
            cells: Vec::with_capacity(x),
            links: HashMap::new()
        };

        for i in 0..x {
            let mut row = Vec::with_capacity(y);
            for j in 0..y {
                row.push(T::new(i, j));

            }
            grid.cells.push(row);
        }

        return grid;
    }

    pub fn print_ascii(&self) {
        print!("{}", self.to_string());
    }

    pub fn print_json(&self) {
        println!("{}", self.to_json());
    }

    pub fn generate_aldous_broder(&mut self) {
        generator::aldous_broder::generate(self)
    }

    pub fn generate_binary(&mut self) {
        generator::binary::generate(self)
    }

    pub fn generate_sidewinder(&mut self) {
        generator::sidewinder::generate(self)
    }

    pub fn is_linked_indices(&self, x1: usize, y1: usize, x2: usize, y2: usize) -> bool {
        match self.links.get(&(x1, y1)) {
            Some(set) => {
                match set.get(&(x2, y2)) {
                    Some(_) => {
                        true
                    },
                    _ => {
                        false
                    }
                }
            },
            None => false
        }
    }

    pub fn is_linked(&self, cell1: &T, cell2: &T) -> bool {
        self.is_linked_indices(cell1.x(), cell1.y(), cell2.x(), cell2.y())
    }

    pub fn link(&mut self, cell1: &T, cell2: &T) {
        self.link_indices(cell1.x(), cell1.y(), cell2.x(), cell2.y());
    }

    pub fn link_indices(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        self.link_pair(x1, y1, x2, y2);
        self.link_pair(x2, y2, x1, y1);
    }

    pub fn link_pair(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        match self.links.contains_key(&(x1, y1)) {
            true => {
                self.links.get_mut(&(x1, y1)).unwrap().insert((x2, y2));
            },
            false => {
                let mut set: BTreeSet<(usize, usize)> = BTreeSet::new();
                set.insert((x2, y2));
                self.links.insert((x1, y1), set);
            }
        }
    }

    pub fn neighbors(&self, cell: &T) -> Vec<T> {
        self.neighbors_indices(cell.x(), cell.y())
    }

    pub fn neighbors_indices(&self, x: usize, y: usize) -> Vec<T> {
        let mut res = Vec::new();

        if x > 0 {
            res.push(self.cells[x - 1][y].clone());
        }

        if x < self.x - 1 {
            res.push(self.cells[x + 1][y].clone());
        }

        if y > 0 {
            res.push(self.cells[x][y - 1].clone());
        }

        if y < self.y - 1 {
            res.push(self.cells[x][y + 1].clone());
        }

        return res;
    }

    pub fn random_cell(&self) -> T {
        let between_x = Range::new(0, self.x);
        let between_y = Range::new(0, self.y);
        let mut rng = rand::thread_rng();

        let x = between_x.ind_sample(&mut rng);
        let y = between_y.ind_sample(&mut rng);

        self.cells[x][y].clone()
    }

    pub fn size(&self) -> usize {
        self.x * self.y
    }

    pub fn to_json(&self) -> String {
        output::json::format(self)
    }

    pub fn to_png(&self, cell_size: u32, wall_size: u32) {
        output::png::format(self, cell_size, wall_size)
    }

    pub fn to_string(&self) -> String {
        output::ascii::format(self)
    }

    pub fn unlink(&mut self, cell1: &T, cell2: &T) {
        self.unlink_indices(cell1.x(), cell1.y(), cell2.x(), cell2.y());
    }

    pub fn unlink_indices(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        self.unlink_pair(x1, y1, x2, y2);
        self.unlink_pair(x2, y2, x1, y1);
    }

    pub fn unlink_pair(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        let mut remove = false;
        match self.links.contains_key(&(x1, y1)) {
            true => {
                let set = self.links.get_mut(&(x1, y1)).unwrap();
                set.remove(&(x2, y2));
                if set.is_empty() {
                    remove = true;
                }
            },
            _ => {}
        }

        if remove {
            self.links.remove(&(x1, y1));
        }
    }

    pub fn visit<F>(&mut self, mut f: F)
        where F: FnMut(&mut Grid<T>, &T)
    {
        let mut grid = self;
        for x in 0..grid.x {
            for y in 0..grid.y {
                let cell = grid[x][y].clone();
                f(grid, &cell);
            }
        }
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }
}

impl <T> Index<usize> for Grid<T>
    where T: Cell
{
    type Output = Vec<T>;

    fn index<'a>(&'a self, index: usize) -> &'a Vec<T> {
        &self.cells[index]
    }
}

impl <T> IndexMut<usize> for Grid<T>
    where T: Cell
{
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut Vec<T> {
        &mut self.cells[index]
    }
}

