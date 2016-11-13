use std::iter;

use super::super::types::cell::Cell;
use super::super::types::grid::Grid;

pub fn format<T>(grid: &Grid<T>) -> String
    where T: Cell + Clone
{
    let mut res = String::new();
    res += "+";
    res += &iter::repeat("---+").take(grid.x()).collect::<String>()[..];
    res += "\n";

    for y in 0..grid.y() {
        let mut top = "|".to_string();
        let mut bottom = "+".to_string();

        for x in 0..grid.x() {
            top += &grid.cells[x][y].to_string()[..];

            match grid.is_linked_indices(x, y, x + 1, y) {
                true => top += " ",
                false => top += "|"
            }

            match grid.is_linked_indices(x, y, x, y + 1) {
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
