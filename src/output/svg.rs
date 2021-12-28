use svg::node::element::Rectangle;
use svg::node::element::Style;
use svg::Document;

use super::super::types::cell::Cell;
use super::super::types::grid::Grid;

pub fn format<T>(
    grid: &Grid<T>,
    cell_size: u32,
    wall_size: u32,
    color_cell: &[u8; 3],
    color_wall: &[u8; 3],
    output_filename: &'static str,
) where
    T: Cell + Clone,
{
    let img_x = (grid.x() as u32 * cell_size) + (grid.x() as u32 + 1) * wall_size;
    let img_y = (grid.y() as u32 * cell_size) + (grid.y() as u32 + 1) * wall_size;

    let style = format!(
        r#"
    	svg {{
    		background-color: rgba({},{},{},1.0);
    	}}

    	rectangle {{
    		background-color: rgba({},{},{},1.0);
    	}}
    "#,
        color_cell[0], color_cell[1], color_cell[2], color_wall[0], color_wall[1], color_wall[2]
    );

    let mut document = Document::new()
        .set("width", img_x)
        .set("height", img_y)
        .set("viewBox", (0, 0, img_x, img_y))
        .add(Style::new(style));

    info!(
        "Generating {:?}, size: {}x{} px",
        output_filename, img_x, img_y
    );

    // Top Wall
    document = document.add(
        Rectangle::new()
            .set("x", 0)
            .set("y", 0)
            .set("width", img_x)
            .set("height", wall_size),
    );

    // Left Wall
    document = document.add(
        Rectangle::new()
            .set("x", 0)
            .set("y", 0)
            .set("width", wall_size)
            .set("height", img_y),
    );

    // Draw all the vertical lines (as one long line between the maze doors)
    let size_x = wall_size;
    let size_y = cell_size + 2 * wall_size;

    for x in 0..grid.x() {
        let start_x = (x as u32 + 1) * (cell_size + wall_size);
        let mut start_y = None;

        for y in 0..grid.y() {
            let cell = &grid[x][y];
            let current_y = y as u32 * (cell_size + wall_size);

            let right = grid.is_linked_indices(cell.x(), cell.y(), cell.x() + 1, cell.y());
            if right {
                // The wall would stop here, so draw if needed.
                if let Some(start_y) = start_y {
                    debug!(
                        "right: ({}, {}), start: ({}, {}), size({}, {})",
                        x, y, start_x, start_y, size_x, size_y
                    );
                    document = document.add(
                        Rectangle::new()
                            .set("x", start_x)
                            .set("y", start_y)
                            .set("width", size_x)
                            .set("height", current_y - start_y + wall_size),
                    );
                }
                start_y = None;
            } else if start_y.is_none() {
                start_y = Some(current_y);
            }
        }

        if let Some(start_y) = start_y {
            let current_y = grid.y() as u32 * (cell_size + wall_size);

            document = document.add(
                Rectangle::new()
                    .set("x", start_x)
                    .set("y", start_y)
                    .set("width", size_x)
                    .set("height", current_y - start_y + wall_size),
            );
        }
    }

    // Draw all the horiztonal lines (as one long line between the maze doors)
    let size_x = cell_size + 2 * wall_size;
    let size_y = wall_size;

    for y in 0..grid.y() {
        let mut start_x = None;
        let start_y = (y as u32 + 1) * (cell_size + wall_size);

        for x in 0..grid.x() {
            let cell = &grid[x][y];
            let current_x = x as u32 * (cell_size + wall_size);

            // Bottom - Horizontal
            let bottom = grid.is_linked_indices(cell.x(), cell.y(), cell.x(), cell.y() + 1);
            if bottom {
                if let Some(start_x) = start_x {
                    debug!(
                        "bottom: ({}, {}), start: ({}, {}), size({}, {})",
                        x, y, start_x, start_y, size_x, size_y
                    );
                    document = document.add(
                        Rectangle::new()
                            .set("x", start_x)
                            .set("y", start_y)
                            .set("width", current_x - start_x + wall_size)
                            .set("height", size_y),
                    );
                }
                start_x = None;
            } else if start_x.is_none() {
                start_x = Some(current_x);
            }
        }
        if let Some(start_x) = start_x {
            let current_x = grid.x() as u32 * (cell_size + wall_size);
            document = document.add(
                Rectangle::new()
                    .set("x", start_x)
                    .set("y", start_y)
                    .set("width", current_x - start_x + wall_size)
                    .set("height", size_y),
            );
        }
    }

    svg::save(output_filename, &document).unwrap();
}
