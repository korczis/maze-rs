extern crate image;
extern crate imageproc;

use image::{Rgb, RgbImage};
use imageproc::rect::Rect;
use imageproc::drawing::draw_filled_rect_mut;

use super::super::types::cell::Cell;
use super::super::types::grid::Grid;

pub fn format<T>(grid: &Grid<T>, cell_size: u32, wall_size: u32)
    where T: Cell + Clone
{
    let imgx = (grid.x() as u32 * cell_size) + (grid.x() as u32 + 1) * wall_size;
    let imgy = (grid.y() as u32 * cell_size) + (grid.y() as u32 + 1) * wall_size;

    let mut img = RgbImage::new(imgx, imgy);

    let wall_color = Rgb([255, 0, 0]);

    // Top
    draw_filled_rect_mut(&mut img, Rect::at(0, 0).of_size(imgx, wall_size), wall_color);

    // Left
    draw_filled_rect_mut(&mut img, Rect::at(0, 0).of_size(wall_size, (imgy - wall_size) as u32), wall_color);

    // Bottom
    draw_filled_rect_mut(&mut img, Rect::at(0, (imgy - wall_size) as i32).of_size(imgx, wall_size), wall_color);

    // Right
    draw_filled_rect_mut(&mut img, Rect::at((imgy - wall_size) as i32, 0).of_size(wall_size, (imgy - wall_size) as u32), wall_color);

    // Cells
    for y in 0..grid.y() {
        for x in 0..grid.x() {
            let cell = &grid[x][y];

            let right = grid.is_linked_indices(cell.x(), cell.y(), cell.x() + 1, cell.y());
            if !right {
                let startx = (x + 1) as i32 * cell_size as i32 + (x + 1) as i32 * wall_size as i32;
                let starty = y as i32 * cell_size as i32 + y as i32 * wall_size as i32;
                draw_filled_rect_mut(&mut img, Rect::at(startx, starty).of_size(wall_size, (cell_size + 2 * wall_size) as u32), wall_color);
            }

            let bottom = grid.is_linked_indices(cell.x(), cell.y(), cell.x(), cell.y() + 1);
            if !bottom {
                let startx = x as i32 * cell_size as i32 + x as i32 * wall_size as i32;
                let starty = (y + 1) as i32 * cell_size as i32 + (y + 1) as i32 * wall_size as i32;
                draw_filled_rect_mut(&mut img, Rect::at(startx, starty).of_size((cell_size + 2 * wall_size) as u32, wall_size), wall_color);
            }
        }
    }

    img.save("output.png").unwrap();
}
