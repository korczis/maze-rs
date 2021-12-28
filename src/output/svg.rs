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
    for x in 0..grid.x() {
        let start_x = (x as u32 + 1) * (cell_size + wall_size);

        let mut y = 0;
        while y < grid.y() {
            let right = grid.is_linked_indices(x, y, x + 1, y);
            if !right {
                let start_y = y as u32 * (cell_size + wall_size);

                // Draw until we get to the gap
                let mut last_y = grid.y();
                while y < last_y {
                    y += 1;
                    if grid.is_linked_indices(x, y, x + 1, y) {
                        last_y = y;
                        break;
                    }
                }

                let end_y = last_y as u32 * (cell_size + wall_size);

                document = document.add(
                    Rectangle::new()
                        .set("x", start_x)
                        .set("y", start_y)
                        .set("width", wall_size)
                        .set("height", end_y - start_y + wall_size),
                );
            }

            y += 1;
        }
    }

    // Draw all the horiztonal lines (as one long line between the maze doors)
    for y in 0..grid.y() {
        let start_y = (y as u32 + 1) * (cell_size + wall_size);

        let mut x = 0;
        while x < grid.x() {
            let bottom = grid.is_linked_indices(x, y, x, y + 1);
            if !bottom {
                let start_x = x as u32 * (cell_size + wall_size);

                // Draw until we get to the gap
                let mut last_x = grid.x();
                while x < last_x {
                    x += 1;
                    if grid.is_linked_indices(x, y, x, y + 1) {
                        last_x = x;
                        break;
                    }
                }

                let end_x = last_x as u32 * (cell_size + wall_size);

                document = document.add(
                    Rectangle::new()
                        .set("x", start_x)
                        .set("y", start_y)
                        .set("width", end_x - start_x + wall_size)
                        .set("height", wall_size),
                );
            }

            x += 1;
        }
    }

    svg::save(output_filename, &document).unwrap();
}
