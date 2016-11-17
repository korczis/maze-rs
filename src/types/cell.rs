// use std::collections::HashMap;

pub trait Cell {
    fn new(x: usize, y: usize) -> Self;
    fn to_string(&self) -> String;
    fn x(&self) -> usize;
    fn y(&self) -> usize;
    // fn clone(&self) -> Self;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BaseCell {
    x: usize,
    y: usize
}

const EMPTY_CELL: &'static str = "   ";

impl Cell for BaseCell {
    fn new(x: usize, y: usize) -> BaseCell {
        BaseCell {
            x: x,
            y: y
        }
    }

    fn to_string(&self) -> String {
        EMPTY_CELL.to_string()
    }

    fn x(&self) -> usize {
        self.x
    }

    fn y(&self) -> usize {
        self.y
    }
}
