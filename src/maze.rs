use crate::vec::Vecu;
use crate::{MAZE_HEIGHT_U8, MAZE_SIZE, MAZE_WIDTH_U8};
use core::slice::Iter;

/// Represents the maze
pub struct Maze {
    segments: [Segment; MAZE_SIZE],
}

/// Calculates the distance to the specified point
fn maze_calc_distance(x: u8, y: u8, cx: i8, cy: i8) -> u8 {
    (i8::abs((x as i8) - cx) + i8::abs((y as i8) - cy)) as u8
}

impl Maze {
    /// Creates a new maze instance.
    pub fn new() -> Self {
        let mut points = [Segment::new(); MAZE_SIZE];

        for x in 0..MAZE_WIDTH_U8 {
            for y in 0..MAZE_HEIGHT_U8 {
                let distances = [
                    maze_calc_distance(x, y, 8, 8),
                    maze_calc_distance(x, y, 7, 8),
                    maze_calc_distance(x, y, 8, 7),
                    maze_calc_distance(x, y, 7, 7),
                ];

                points[(x + y * MAZE_WIDTH_U8) as usize] = Segment {
                    pos: Vecu { x, y },
                    distance: *distances.iter().min().unwrap(),
                    walls: [false, false, false, false],
                };
            }
        }

        Maze { segments: points }
    }

    /// Returns the segment at `x, y`.
    pub fn segment(&self, x: u8, y: u8) -> Segment {
        self.segments[(x + y * MAZE_WIDTH_U8) as usize]
    }

    /// Returns the segment at `x, y`.
    pub fn segment_vec(&self, pos: Vecu) -> Segment {
        self.segment(pos.x, pos.y)
    }

    /// Updates the walls of the segment at `x, y` to the specified array.
    pub fn update_walls(&mut self, x: u8, y: u8, walls: [bool; 4]) {
        let i = (x + y * MAZE_WIDTH_U8) as usize;
        let existing = self.segments[i];

        self.segments[i] = Segment {
            pos: existing.pos,
            distance: existing.distance,
            walls,
        };
    }

    /// Updates the distance of the segment at `x, y` to the specified value.
    pub fn update_distance(&mut self, x: u8, y: u8, distance: u8) {
        let i = (x + y * MAZE_WIDTH_U8) as usize;
        let existing = self.segments[i];

        self.segments[i] = Segment {
            pos: existing.pos,
            distance,
            walls: existing.walls,
        };
    }
}

impl Default for Maze {
    fn default() -> Self {
        Self::new()
    }
}

impl core::fmt::Debug for Maze {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for y in 0..MAZE_HEIGHT_U8 {
            for x in 0..MAZE_WIDTH_U8 {
                let distance = self.segment(x, y).distance;
                if distance < 10 {
                    write!(f, "{:?}  ", distance)?;
                } else {
                    write!(f, "{:?} ", distance)?;
                }
            }
            writeln!(f)?;
        }
        writeln!(f)?;
        Ok(())
    }
}

/// Represents a point on the grid.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Segment {
    pos: Vecu,
    pub distance: u8,
    pub walls: [bool; 4],
}

/// The relative direction.
#[derive(PartialEq, Debug)]
pub enum Relative {
    North,
    East,
    South,
    West,
}

impl Relative {
    pub fn iter() -> Iter<'static, Relative> {
        static DIRECTIONS: [Relative; 4] = [
            Relative::North,
            Relative::East,
            Relative::South,
            Relative::West,
        ];
        DIRECTIONS.iter()
    }
}

impl Segment {
    /// Creates a new default Segment.
    pub fn new() -> Self {
        Segment {
            pos: Vecu::new(),
            distance: u8::MAX,
            walls: [false, false, false, false],
        }
    }

    /// Whether this segment is a dead end.
    /// A dead end is defined as a segment with 3 walls and 1 entrance.
    pub fn is_dead_end(&self) -> bool {
        self.walls.iter().filter(|it| **it).count() == 3
    }

    /// Whether this segment is straight.
    /// A segment is straight when there are 2 walls and 1 exit and 1 entrance.
    pub fn is_straight(&self) -> bool {
        self.walls.iter().filter(|it| **it).count() == 2
    }

    /// Returns the position of this segment.
    pub fn pos(&self) -> Vecu {
        self.pos
    }

    /// Returns the segment relative to this one by direction `relative`.
    ///
    /// ### Arguments
    ///
    /// - `maze` - A maze ref.
    /// - `relative` - The direction.
    pub fn relative(&self, maze: &Maze, relative: &Relative) -> Option<Segment> {
        let (x, y) = (self.pos.x, self.pos.y);

        match *relative {
            Relative::North if y > 0 => Some(maze.segment(x, y - 1)),
            Relative::South if y + 1 < MAZE_HEIGHT_U8 => Some(maze.segment(x, y + 1)),
            Relative::West if x > 0 => Some(maze.segment(x - 1, y)),
            Relative::East if x + 1 < MAZE_WIDTH_U8 => Some(maze.segment(x + 1, y)),
            _ => None,
        }
    }
}

impl Default for Segment {
    fn default() -> Self {
        Self::new()
    }
}
