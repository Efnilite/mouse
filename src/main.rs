use crate::maze::Maze;
use crate::vec2::{Vec2f, Vec2i};

mod vec2;
mod maze;

/// The maze width.
const MAZE_WIDTH: u16 = 16;

/// The maze height.
const MAZE_HEIGHT: u16 = 16;

/// The maze size.
const MAZE_SIZE: usize = (MAZE_WIDTH * MAZE_HEIGHT) as usize;

/// The maze wall thickness in meters.
const MAZE_WALL_THICKNESS: f32 = 0.02;

fn main() {
    let fpos = Vec2f { x: 0f32, y: 0f32 };
    let ipos = Vec2i { x: 0, y: 0};
    let maze = Maze::new();

    println!("Initialized with");
    println!("{:?}", fpos);
    println!("{:?}", ipos);
    println!("{:?}", maze);

    println!("Hello, world!");
}
