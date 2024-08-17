/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/stara-rs
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */
use crate::grid::Grid;
#[cfg(test)]
use crate::prelude::*;
use int_math::VectorU;

#[test]
fn test_horizontal_path_no_obstacles() {
    let start = VectorU::new(0, 0);
    let goal = VectorU::new(5, 0);
    let width = 6;
    let height = 1;
    let size = VectorU {
        x: width,
        y: height,
    };

    let grid = Grid::new(size, 1);
    let path = AStar::search(start, goal, &grid).unwrap();

    let expected_path = vec![
        VectorU::new(0, 0),
        VectorU::new(1, 0),
        VectorU::new(2, 0),
        VectorU::new(3, 0),
        VectorU::new(4, 0),
        VectorU::new(5, 0),
    ];

    assert_eq!(path, expected_path);
}

#[test]
fn test_path_around_obstacle() {
    let start = VectorU::new(0, 0);
    let goal = VectorU::new(5, 0);
    let width = 6;
    let height = 3;
    let size = VectorU {
        x: width,
        y: height,
    };

    let mut grid = Grid::new(size, 1);
    grid.set_cost(VectorU::new(3, 0), IMPASSABLE);
    grid.set_cost(VectorU::new(3, 1), IMPASSABLE);

    let path = AStar::search(start, goal, &grid).unwrap();

    let expected_path = vec![
        VectorU::new(0, 0),
        VectorU::new(1, 0),
        VectorU::new(2, 0),
        VectorU::new(2, 1),
        VectorU::new(2, 2),
        VectorU::new(3, 2),
        VectorU::new(4, 2),
        VectorU::new(5, 2),
        VectorU::new(5, 1),
        VectorU::new(5, 0),
    ];

    assert_eq!(path, expected_path);
}

#[test]
fn test_no_path() {
    let start = VectorU::new(0, 0);
    let goal = VectorU::new(5, 0);
    let width = 6;
    let height = 1;

    let size = VectorU {
        x: width,
        y: height,
    };

    let mut grid = Grid::new(size, 1);
    grid.set_cost(VectorU::new(3, 0), IMPASSABLE);
    grid.set_cost(VectorU::new(4, 0), IMPASSABLE);
    grid.set_cost(VectorU::new(5, 0), IMPASSABLE);

    let path = AStar::search(start, goal, &grid);

    assert!(path.is_none());
}

#[test]
fn test_shortest_path_with_high_cost() {
    let start = VectorU::new(0, 0);
    let goal = VectorU::new(4, 4);
    let width = 5;
    let height = 5;

    let size = VectorU {
        x: width,
        y: height,
    };

    let mut grid = Grid::new(size, 1);
    grid.set_cost(VectorU::new(2, 1), 10);
    grid.set_cost(VectorU::new(2, 2), 10);
    grid.set_cost(VectorU::new(2, 3), 10);

    let path = AStar::search(start, goal, &grid).unwrap();

    let expected_path = vec![
        VectorU::new(0, 0),
        VectorU::new(1, 0),
        VectorU::new(1, 1),
        VectorU::new(1, 2),
        VectorU::new(1, 3),
        VectorU::new(1, 4),
        VectorU::new(2, 4),
        VectorU::new(3, 4),
        VectorU::new(4, 4),
    ];

    assert_eq!(path, expected_path);
}

#[test]
fn test_two_agents() {
    let start = VectorU::new(0, 0);
    let goal = VectorU::new(4, 4);
    let width = 5;
    let height = 5;

    let size = VectorU {
        x: width,
        y: height,
    };

    let mut grid = Grid::new(size, 1);
    grid.set_cost(VectorU::new(2, 1), 10);
    grid.set_cost(VectorU::new(2, 2), 10);
    grid.set_cost(VectorU::new(2, 3), 10);

    {
        let path1 = AStar::search(start, goal, &grid).unwrap();

        let expected_path1 = vec![
            VectorU::new(0, 0),
            VectorU::new(1, 0),
            VectorU::new(1, 1),
            VectorU::new(1, 2),
            VectorU::new(1, 3),
            VectorU::new(1, 4),
            VectorU::new(2, 4),
            VectorU::new(3, 4),
            VectorU::new(4, 4),
        ];

        assert_eq!(path1, expected_path1);
    }

    {
        let start2 = VectorU::new(0, 0);
        let goal2 = VectorU::new(4, 0);
        let path2 = AStar::search(start2, goal2, &grid).unwrap();

        let expected_path2 = vec![
            VectorU::new(0, 0),
            VectorU::new(1, 0),
            VectorU::new(2, 0),
            VectorU::new(3, 0),
            VectorU::new(4, 0),
        ];

        assert_eq!(path2, expected_path2);
    }
}
