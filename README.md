# A* Pathfinding Library in Rust

Rust :crab: library implementing the [A* pathfinding algorithm](https://en.wikipedia.org/wiki/A*_search_algorithm).
`A*` is used for efficiently finding the shortest path in a grid, mainly used for games.

## Installation

Add the following to your `Cargo.toml` under `[dependencies]` to use it in your project:

```toml
[dependencies]
stara = "0.0.2"
```

## Usage

Here’s an example of how to use the A* pathfinding algorithm with this library:

```rust
use stara::prelude::*;

fn main() {
    let start = VectorU::new(0, 0);
    let goal = VectorU::new(5, 5);
    let size = VectorU::new(7, 7);

    let mut grid = Grid::new(size, 1);

    // Setting an obstacle
    grid.set_cost(VectorU::new(3, 3), IMPASSABLE);

    let maybe_path = AStar::search(start, goal, &grid);
    if let Some(path) = maybe_path {
        println!("Path found: {:?}", path);
    } else {
        println!("No path found.");
    }
}
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.
