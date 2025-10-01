use crate::maze::Maze;
use crate::path::Path;
use crate::pathfinder::next;
use crate::vec::{Vecf, Veci};

mod vec;
mod maze;
mod path;
mod pathfinder;

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
    let ipos = Veci { x: 0, y: 0};
    let maze = Maze::new();
    let mut path = Path::new();

    println!("Initialized with");
    println!("{:?}", fpos);
    println!("{:?}", ipos);
    println!("{:?}", maze);
    println!("{:?}", path);

    loop {
        let next = next(&maze, &path);
        path.append(next);

        println!("Next: {:?}", next);

        if next.distance == 0 {
            break;
        }
    }

}
