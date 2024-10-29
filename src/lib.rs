/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/stara-rs
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */
pub mod prelude;
#[cfg(test)]
mod test;

pub mod bresenham;
pub mod grid;
pub mod los;
mod node;
pub mod reduce;

use node::Node;

use crate::grid::Grid;
use int_math::VectorU;
use std::collections::{BinaryHeap, HashMap, HashSet};

/// The movement cost to go into a cell
pub type Cost = u8;
type Score = u16;

/// Represents the cost of an impassable tile in the grid.
pub const IMPASSABLE: u8 = u8::MAX;

#[allow(unused)]

fn neighbors(pos: VectorU) -> Vec<VectorU> {
    let mut v = vec![
        VectorU {
            x: pos.x + 1,
            y: pos.y,
        },
        VectorU {
            x: pos.x,
            y: pos.y + 1,
        },
    ];

    if pos.x > 0 {
        v.push(VectorU {
            x: pos.x - 1,
            y: pos.y,
        });
    }

    if pos.y > 0 {
        v.push(VectorU {
            x: pos.x,
            y: pos.y - 1,
        });
    }

    v
}

/// The `AStar` struct implements the A* pathfinding algorithm on a 2D grid.
/// It finds the shortest path from a start point to a goal point, considering
/// the cost of each tile in the grid.
#[allow(unused)]
pub struct AStar {}

impl AStar {
    /// Calculates the heuristic estimate of the cost from a given point to the goal.
    /// Currently using the [Manhattan Distance](https://en.wikipedia.org/wiki/Taxicab_geometry).
    #[allow(unused)]
    fn heuristic(p1: VectorU, p2: VectorU) -> Score {
        ((p1.x as i32 - p2.x as i32).abs() + (p1.y as i32 - p2.y as i32).abs()) as Score
    }

    /// Performs the A* search algorithm to find the shortest path from the start point to the goal.
    ///
    /// # Arguments
    ///
    /// * `start` - A [`VectorU`] representing the start position in the grid.
    /// * `goal` - A [`VectorU`] representing the goal position in the grid.
    /// * `grid` - The `Grid` on which the pathfinding will be performed.
    ///
    /// # Returns
    ///
    /// Returns an `Option<Vec<VectorU>>` containing the sequence of points representing the path from start to goal.
    /// If no path is found, returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use int_math::VectorU;
    /// use stara::prelude::*;
    ///
    /// let grid = Grid::new(VectorU::new(10,10), 1);
    /// let path = AStar::search(VectorU::new(0,0), VectorU::new(9,9), &grid);
    ///
    /// if let Some(path) = path {
    ///     println!("Path found: {:?}", path);
    /// } else {
    ///     println!("No path found.");
    /// }
    /// ```
    #[allow(unused)]
    pub fn search(start: VectorU, goal: VectorU, grid: &Grid) -> Option<Vec<VectorU>> {
        let mut open_set = BinaryHeap::new();
        let mut open_set_positions = HashMap::new();
        let mut closed_set = HashSet::new();

        let start_node = Node {
            position: start,
            g: 0,
            f: Self::heuristic(start, goal),
            parent: None,
        };

        open_set.push(start_node.clone());
        open_set_positions.insert(start, start_node);

        while let Some(current_node) = open_set.pop() {
            let current_position = current_node.position;
            open_set_positions.remove(&current_position);

            if current_position == goal {
                let mut path = vec![current_position];
                let mut current = current_node;
                while let Some(parent_node) = current.parent {
                    path.push(parent_node.position);
                    current = *parent_node;
                }
                path.reverse();
                return Some(path);
            }

            closed_set.insert(current_position);

            for neighbor_position in neighbors(current_position) {
                if !grid.in_bounds(neighbor_position) {
                    continue;
                }
                if closed_set.contains(&neighbor_position) {
                    continue;
                }

                let neighbor_cost = grid.cost(neighbor_position);
                if neighbor_cost == IMPASSABLE {
                    continue;
                }

                let tentative_g_score = &current_node.g + 1 + neighbor_cost as Score;

                if let Some(open_neighbor_node) = open_set_positions.get(&neighbor_position) {
                    // If we already have a better path to this neighbor node, skip this path to the neighbor
                    if tentative_g_score >= open_neighbor_node.g {
                        continue;
                    }
                }

                if open_set_positions.contains_key(&neighbor_position) {
                    continue;
                }

                let f_score = tentative_g_score + Self::heuristic(neighbor_position, goal);

                let neighbor_node = Node {
                    position: neighbor_position,
                    g: tentative_g_score,
                    f: f_score,
                    parent: Some(Box::new(current_node.clone())),
                };

                open_set.push(neighbor_node.clone());
                open_set_positions.insert(neighbor_position, neighbor_node);
            }
        }

        None
    }
}
