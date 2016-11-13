extern crate image;
extern crate imageproc;

use image::{Rgb, RgbImage};
use imageproc::rect::Rect;
use imageproc::drawing::draw_filled_rect_mut;

use super::super::types::cell::Cell;
use super::super::types::grid::Grid;

pub fn format<T>(grid: &Grid<T>, cell_size: u32, wall_size: u32, output_filename: &'static str)
    where T: Cell + Clone
{
    let img_x = (grid.x() as u32 * cell_size) + (grid.x() as u32 + 1) * wall_size;
    let img_y = (grid.y() as u32 * cell_size) + (grid.y() as u32 + 1) * wall_size;

    info!("Generating {:?}, size: {}x{} px", output_filename, img_x, img_y);
    let mut img = RgbImage::new(img_x, img_y);

    let background_color = Rgb([255, 255, 255]);
    let wall_color = Rgb([0, 0, 0]);

    draw_filled_rect_mut(&mut img, Rect::at(0, 0).of_size(img_x, img_y), background_color);

    // Top
    draw_filled_rect_mut(&mut img, Rect::at(0, 0).of_size(img_x, wall_size), wall_color);

    // Left
    draw_filled_rect_mut(&mut img, Rect::at(0, 0).of_size(wall_size, (img_y - wall_size) as u32), wall_color);

    // Cells
    for x in 0..grid.x() {
        for y in 0..grid.y() {
            let cell = &grid[x][y];

            let right = grid.is_linked_indices(cell.x(), cell.y(), cell.x() + 1, cell.y());
            if !right {
                let start_x = (x + 1) as i32 * cell_size as i32 + (x + 1) as i32 * wall_size as i32;
                let start_y = y as i32 * cell_size as i32 + y as i32 * wall_size as i32;
                let size_x = wall_size;
                let size_y = (cell_size + 2 * wall_size) as u32;
                debug!("right: ({}, {}), start: ({}, {}), size({}, {})", x, y, start_x, start_y, size_x, size_y);
                draw_filled_rect_mut(&mut img, Rect::at(start_x, start_y).of_size(size_x, size_y), wall_color);
            }

            let bottom = grid.is_linked_indices(cell.x(), cell.y(), cell.x(), cell.y() + 1);
            if !bottom {
                let start_x = x as i32 * cell_size as i32 + x as i32 * wall_size as i32;
                let start_y = (y + 1) as i32 * cell_size as i32 + (y + 1) as i32 * wall_size as i32;
                let size_x = (cell_size + 2 * wall_size) as u32;
                let size_y = wall_size;
                debug!("bottom: ({}, {}), start: ({}, {}), size({}, {})", x, y, start_x, start_y, size_x, size_y);
                draw_filled_rect_mut(&mut img, Rect::at(start_x, start_y).of_size(size_x, size_y), wall_color);
            }
        }
    }

    img.save(output_filename).unwrap();
}
