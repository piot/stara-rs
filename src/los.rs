use crate::bresenham::bresenham_line;
use crate::prelude::Grid;
use crate::Cost;
use int_math::VectorU;

pub fn has_line_of_sight(grid: &Grid, start: VectorU, end: VectorU, threshold: Cost) -> bool {
    let line_points = bresenham_line(start.x as i32, start.y as i32, end.x as i32, end.y as i32);

    let mut prev: Option<VectorU> = None;

    for (x, y) in line_points {
        let current_vector = VectorU::new(x as u32, y as u32);

        if !grid.in_bounds(current_vector) {
            return false;
        }

        if grid.cost(current_vector) >= threshold {
            return false;
        }

        if let Some(prev_position) = prev {
            let dx = (x - prev_position.x as i32).abs();
            let dy = (y - prev_position.y as i32).abs();

            if dx == 1 && dy == 1 {
                // Diagonal movement detected, check all adjacent cells
                let adj1 = VectorU::new(prev_position.x, y as u32);
                let adj2 = VectorU::new(x as u32, prev_position.y);

                if grid.in_bounds(adj1) && grid.cost(adj1) >= threshold {
                    return false;
                }
                if grid.in_bounds(adj2) && grid.cost(adj2) >= threshold {
                    return false;
                }
            }
        }

        prev = Some(current_vector);
    }

    true
}
