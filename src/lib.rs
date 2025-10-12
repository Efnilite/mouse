/// The maze width.
pub const MAZE_WIDTH_U8: u8 = 16;
pub const MAZE_WIDTH_USIZE: usize = 16;

/// The maze height.
pub const MAZE_HEIGHT_U8: u8 = 16;
pub const MAZE_HEIGHT_USIZE: usize = 16;

/// The maze size.
pub const MAZE_SIZE: usize = MAZE_WIDTH_USIZE * MAZE_HEIGHT_USIZE;

/// The size of a block of a maze.
pub const MAZE_BLOCK_M: f64 = 0.18;

mod map;
pub mod maze;
pub mod path;
pub mod pathfinder;
pub mod vec;
