use crate::los::has_line_of_sight;
use crate::prelude::Grid;
use crate::IMPASSABLE;
use int_math::VectorU;

pub fn reduce(grid: &Grid, path: &[VectorU]) -> Vec<VectorU> {
    if path.is_empty() {
        return vec![];
    }
    if path.len() == 1 {
        return vec![*path.first().unwrap()];
    }

    let mut result = vec![];

    let mut last_inserted_position = *path.first().unwrap();
    let mut last_visible_point = last_inserted_position;
    let mut last_inserted_index = 0;

    result.push(last_inserted_position);

    for (i, &pos) in path.iter().enumerate().skip(1) {
        if has_line_of_sight(grid, last_inserted_position, pos, IMPASSABLE) {
            last_visible_point = pos;
            continue;
        }

        last_inserted_position = last_visible_point;
        last_inserted_index = i;
        result.push(last_inserted_position);

        last_visible_point = pos;
    }

    if last_inserted_index != path.len() - 1 {
        result.push(last_visible_point);
    }

    result
}
