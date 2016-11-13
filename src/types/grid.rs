extern crate rand;
extern crate serde_json;

use serde_json::Map;

use std::collections::BTreeSet;
use std::collections::HashMap;
use std::iter;
use std::ops::{Index, IndexMut};

use rand::Rng;
use rand::distributions::{IndependentSample, Range};

use super::cell::Cell;

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
        let mut cell = self.random_cell();
        let mut unvisited = self.size() - 1;

        while unvisited > 0 {
            let neighbors = self.neighbors(&cell);
            let neighbor = rand::thread_rng().choose(&neighbors).unwrap();

            if !self.links.contains_key(&(neighbor.x(), neighbor.y())) {
                self.link(&cell, &neighbor);
                unvisited -= 1;
            }

            cell = neighbor.clone();
        }
    }

    pub fn generate_binary(&mut self) {
        self.visit(|grid, cell| {
            let mut cells: Vec<T> = Vec::new();

            if cell.x() < (grid.x - 1) {
                cells.push(grid[cell.x() + 1][cell.y()].clone());
            }

            if cell.y() < (grid.y - 1) {
                cells.push(grid[cell.x()][cell.y() + 1].clone());
            }

            if cells.len() > 0 {
                grid.link(cell, rand::thread_rng().choose(&cells).unwrap());
            }
        });
    }

    pub fn generate_sidewinder(&mut self) {
        for y in 0..self.y {
            let mut cells: Vec<T> = Vec::new();
            for x in 0..self.x {
                cells.push(self.cells[x][y].clone());

                let at_eastern_boundary = x == self.x - 1;
                let at_northern_boundary = y == self.y - 1;

                let should_close_out = at_eastern_boundary || (!at_northern_boundary && rand::thread_rng().gen());

                let mut should_clear = false;
                if should_close_out {
                    let member = rand::thread_rng().choose(&cells).unwrap();
                    if y < (self.y - 1) {
                        self.link_indices(member.x(), member.y(), member.x(), member.y() + 1);
                    }
                    should_clear = true;
                } else {
                    self.link_indices(x, y, x + 1, y);
                }

                if should_clear {
                    cells.clear();
                }
            }
        }
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

        let mut map: Map<String, serde_json::Value> = Map::new();
        let mut links: Vec<serde_json::value::Value> = Vec::new();

        for (k, set) in self.links.iter() {
            for v in set.iter() {
                let mut tuple: Vec<serde_json::value::Value> = Vec::new();
                tuple.push(serde_json::value::Value::U64(k.0 as u64));
                tuple.push(serde_json::value::Value::U64(k.1 as u64));

                tuple.push(serde_json::value::Value::U64(v.0 as u64));
                tuple.push(serde_json::value::Value::U64(v.1 as u64));
                links.push(serde_json::value::Value::Array(tuple));
            }
        }

        map.insert("x".to_string(), serde_json::value::Value::U64(self.x as u64));
        map.insert("y".to_string(), serde_json::value::Value::U64(self.y as u64));
        map.insert("links".to_string(), serde_json::value::Value::Array(links));

        match serde_json::to_string(&map) {
            Ok(json) => {
               return json;
            },
            Err(_) => {
                return String::new()
            }
        }
    }

    pub fn to_string(&self) -> String {
        let mut res = String::new();
        res += "+";
        res += &iter::repeat("---+").take(self.x).collect::<String>()[..];
        res += "\n";

        for y in 0..self.y {
            let mut top = "|".to_string();
            let mut bottom = "+".to_string();

            for x in 0..self.x {
                top += &self.cells[x][y].to_string()[..];

                match self.is_linked_indices(x, y, x + 1, y) {
                    true => top += " ",
                    false => top += "|"
                }

                match self.is_linked_indices(x, y, x, y + 1) {
                    true => bottom += "   +",
                    false => bottom += "---+",
                }
            }

            res += &top[..];
            res += "\n";

            res += &bottom[..];
            res += "\n";
        }

        return res;
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
