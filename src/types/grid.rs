extern crate rand;
extern crate serde_json;

use serde_json::Map;

use std::collections::HashMap;
use std::iter;
use std::ops::{Index, IndexMut};

use rand::Rng;

use super::cell::Cell;

#[derive(Debug, Serialize, Deserialize)]
pub struct Grid {
    x: usize,
    y: usize,
    pub cells: Vec<Vec<Cell>>,
    pub links: HashMap<(usize, usize, usize, usize), bool>
}

impl Grid {
    pub fn new(x: usize, y: usize) -> Grid {
        let mut grid = Grid {
            x: x,
            y: y,
            cells: Vec::with_capacity(x),
            links: HashMap::new()
        };

        for i in 0..x {
            let mut row = Vec::with_capacity(y);
            for j in 0..y {
                row.push(Cell::new(i, j));

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

    pub fn generate_binary(&mut self) {
        self.visit(|grid, cell| {
            let mut cells: Vec<Cell> = Vec::new();

            if cell.x() < (grid.x - 1) {
                cells.push(grid[cell.x() + 1][cell.y()]);
            }

            if cell.y() < (grid.y - 1) {
                cells.push(grid[cell.x()][cell.y() + 1]);
            }

            if cells.len() > 0 {
                grid.link(cell, rand::thread_rng().choose(&cells).unwrap());
            }

            // println!("{:?}", cells);
        });
    }

    pub fn generate_sidewinder(&mut self) {
        for y in 0..self.y {
            let mut cells: Vec<Cell> = Vec::new();
            for x in 0..self.x {
                cells.push(self.cells[x][y]);

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
        match self.links.get(&(x1, y1, x2, y2)) {
            Some(link) => link.clone(),
            None => false
        }
    }

    pub fn is_linked(&self, cell1: &Cell, cell2: &Cell) -> bool {
        self.is_linked_indices(cell1.x(), cell1.y(), cell2.x(), cell2.y())
    }

    pub fn link(&mut self, cell1: &Cell, cell2: &Cell) {
        self.link_indices(cell1.x(), cell1.y(), cell2.x(), cell2.y());
    }

    pub fn link_indices(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        self.links.insert((x1, y1, x2, y2), true);
        self.links.insert((x2, y2, x1, y1), true);
    }

    pub fn to_json(&self) -> String {
        let mut map: Map<String, serde_json::Value> = Map::new();
        let mut links: Vec<serde_json::value::Value> = Vec::new();

        for link in self.links.keys() {
            let mut tuple: Vec<serde_json::value::Value> = Vec::new();
            tuple.push(serde_json::value::Value::U64(link.0 as u64));
            tuple.push(serde_json::value::Value::U64(link.1 as u64));
            tuple.push(serde_json::value::Value::U64(link.2 as u64));
            tuple.push(serde_json::value::Value::U64(link.3 as u64));
            links.push(serde_json::value::Value::Array(tuple));
        };

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
                top += "   ";

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

    pub fn unlink(&mut self, cell1: &Cell, cell2: &Cell) {
        self.unlink_indices(cell1.x(), cell1.y(), cell2.x(), cell2.y());
    }

    pub fn unlink_indices(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        self.links.remove(&(x1, y1, x2, y2));
        self.links.remove(&(x2, y2, x1, y1));
    }

    pub fn visit<F>(&mut self, mut f: F)
        where F: FnMut(&mut Grid, &Cell)
    {
        let mut grid = self;
        for x in 0..grid.x {
            for y in 0..grid.y {
                let cell = grid[x][y];
                f(grid, &cell);
            }
        }
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
