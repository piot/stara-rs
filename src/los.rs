use crate::bresenham::bresenham_line;
use crate::prelude::Grid;
use crate::Cost;
use int_math::VectorU;

pub fn has_line_of_sight(grid: &Grid, start: VectorU, end: VectorU, threshold: Cost) -> bool {
    let line_points = bresenham_line(start.x as i32, start.y as i32, end.x as i32, end.y as i32);

    for (x, y) in line_points {
        let vector = VectorU::new(x as u32, y as u32);
        if grid.cost(vector) >= threshold {
            return false;
        }
    }

    true
}
