/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/stara-rs
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */
pub use crate::Cost;
pub use int_math::VectorU;
use std::fmt::{Display, Formatter};

/// The `Grid` struct represents a 2D grid used in an [A* search algorithm](https://en.wikipedia.org/wiki/A*_search_algorithm).
/// Each cell in the grid has an associated cost, which can be used to
/// calculate the pathfinding cost for traversing through that cell.
pub struct Grid {
    size: VectorU,

    /// A flat vector storing the cost of each cell in the grid.
    /// The vector is indexed based on a 1D representation of the 2D grid.
    cells: Vec<Cost>,
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.size.y {
            writeln!(f)?;
            for x in 0..self.size.x {
                let index = (y * self.size.x + x) as usize;

                let cost = self.cells[index];

                let prefix = if cost <= 127 { "\x1b[32m" } else { "\x1b[91m" };

                write!(f, "{}{:02X}\x1b[0m", prefix, self.cells[index])?;
            }
        }

        Ok(())
    }
}

impl Grid {
    /// Creates a new `Grid` with the specified size and a default cost for all cells.
    ///
    /// # Arguments
    ///
    /// * `size` - A [`VectorU`] representing the width (`x`) and height (`y`) of the grid.
    /// * `default_cost` - The default cost assigned to all cells in the grid.
    ///
    /// # Returns
    ///
    /// Returns a `Grid` instance with all cells initialized to `default_cost`.
    ///
    /// # Examples
    ///
    /// ```
    /// use int_math::VectorU;
    /// use stara::prelude::Grid;
    ///
    /// let grid_size = VectorU::new(10,10);
    /// let grid = Grid::new(grid_size, 1);
    /// ```
    pub fn new(size: VectorU, default_cost: Cost) -> Self {
        let cells = vec![default_cost; (size.x * size.y) as usize];
        Self { size, cells }
    }

    pub fn in_bounds(&self, position: VectorU) -> bool {
        position.x < self.size.x && position.y < self.size.y
    }

    #[inline]
    fn index(&self, position: VectorU) -> usize {
        (position.y * self.size.x + position.x) as usize
    }

    /// Gets the cost of a specific cell in the grid.
    ///
    /// # Arguments
    ///
    /// * `position` - A [`VectorU`] representing the coordinates of the cell.
    ///
    /// # Returns
    ///
    /// Returns the cost associated with the specified cell.
    ///
    /// # Examples
    ///
    /// ```
    /// use int_math::VectorU;
    /// use stara::prelude::Grid;
    ///
    /// let grid_size = VectorU::new(10, 10);
    /// let grid = Grid::new(grid_size, 1);
    /// let cost = grid.cost(VectorU::new(5, 5));
    /// ```
    pub fn cost(&self, position: VectorU) -> Cost {
        self.cells[self.index(position)]
    }

    /// Sets the cost of a specific cell in the grid.
    ///
    /// # Arguments
    ///
    /// * `position` - A [`VectorU`] representing the coordinates of the cell.
    /// * `cost` - The cost to assign to the specified cell.
    ///
    /// # Examples
    ///
    /// ```
    /// use int_math::VectorU;
    /// use stara::prelude::Grid;
    ///
    /// let mut grid = Grid::new(VectorU::new(10, 10), 1);
    /// grid.set_cost(VectorU::new(5, 5), 10);
    /// ```
    #[allow(unused)]
    pub fn set_cost(&mut self, position: VectorU, cost: Cost) {
        let index = self.index(position);
        self.cells[index] = cost;
    }
}
