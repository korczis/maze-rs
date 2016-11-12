#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Cell {
    x: usize,
    y: usize
}

impl Cell {
    pub fn new(x: usize, y: usize) -> Cell {
        Cell {
            x: x,
            y: y
        }
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }
}