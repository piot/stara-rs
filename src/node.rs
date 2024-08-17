/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/stara-rs
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */
use crate::Score;
use int_math::VectorU;
use std::cmp::Ordering;

// A node in a path. A node represents on cell with the accumulated cost (Score) for each cell.
#[derive(Debug, Clone)]
pub(crate) struct Node {
    pub(crate) position: VectorU,         // Position of the cell
    pub(crate) g: Score,                  // Cost from the start node
    pub(crate) f: Score, // Estimated cost from the start to the goal through this node
    pub(crate) parent: Option<Box<Node>>, // The parent data. Is boxed to avoid infinite size
}

// Implement the PartialEq trait to compare Nodes
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

// Implement the Eq trait to compare Nodes
impl Eq for Node {}

// Implement the Ord trait for ordering in the BinaryHeap
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f.partial_cmp(&self.f).unwrap_or(Ordering::Equal)
    }
}

// Implement the PartialOrd trait for ordering in the BinaryHeap
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
