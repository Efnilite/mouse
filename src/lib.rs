#![no_std]
#![no_main]

/// The maze width.
pub const MAZE_WIDTH_U8: u8 = 16;
pub const MAZE_WIDTH_USIZE: usize = 16;

/// The maze height.
pub const MAZE_HEIGHT_U8: u8 = 16;
pub const MAZE_HEIGHT_USIZE: usize = 16;

/// The maze size.
pub const MAZE_SIZE: usize = MAZE_WIDTH_USIZE * MAZE_HEIGHT_USIZE;

pub mod vec;
pub mod path;
pub mod maze;
pub mod pathfinder;
pub mod map;