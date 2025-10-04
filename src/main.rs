use crate::maze::Maze;
use crate::path::Path;
use crate::pathfinder::next_unvisited;
use crate::vec::{Vecf, Veci};

mod maze;
mod path;
mod pathfinder;
mod vec;
mod generator;

/// The maze width.
const MAZE_WIDTH: u8 = 16;

/// The maze height.
const MAZE_HEIGHT: u8 = 16;

/// The maze size.
const MAZE_SIZE: usize = ((MAZE_WIDTH as u16) * (MAZE_HEIGHT as u16)) as usize;

/// The maze wall thickness in meters.
const MAZE_WALL_THICKNESS: f32 = 0.02;

fn main() {
    let fpos = Vecf { x: 0f32, y: 0f32 };
    let ipos = Veci { x: 0, y: 0 };
    let mut maze = Maze::new();
    let mut path = Path::new();

    println!("Initialized with");
    println!("{:?}", fpos);
    println!("{:?}", ipos);
    println!("{:?}", maze);
    println!("{:?}", path);

    maze.update_walls(0, 0, [true, false, true, true]);
    maze.update_walls(1, 0, [true, false, false, false]);
    maze.update_walls(2, 0, [true, true, true, false]);
    maze.update_walls(1, 1, [false, false, false, false]);

    path.append(Veci::new());

    loop {
        let result = pathfinder::next(&maze, &path);

        if result.is_found() {
            let next = result.unwrap();
            path.append(next.pos());

            if next.distance == 0 {
                break;
            }
            continue;
        }

        let mut to_unvisited = next_unvisited(&maze, &path);
        to_unvisited.remove(0); // remove head
        path.append_all(to_unvisited);
    }

    path.optimize();

    println!("{:?}", maze);
    println!("{:?}", path);
}
