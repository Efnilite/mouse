use crate::pathfinder::Target;
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

/// Converts the position to the index for `segments`.
fn xy_to_index(x: u8, y: u8) -> usize {
    (x + y * MAZE_WIDTH_U8) as usize
}

/// Converts the position to the index for `segments`.
fn pos_to_index(pos: Vecu) -> usize {
    xy_to_index(pos.x, pos.y)
}

/// Creates a new Maze with the provided walls.
fn with_walls_fn<T>(target: Target, walls: T) -> Maze
where
    T: Fn(u8, u8) -> [bool; 4],
{
    let mut points = [Segment::new(); MAZE_SIZE];

    let distances;

    match target {
        Target::Center => {
            distances = [
                |x, y| maze_calc_distance(x, y, 8, 8),
                |x, y| maze_calc_distance(x, y, 7, 8),
                |x, y| maze_calc_distance(x, y, 8, 7),
                |x, y| maze_calc_distance(x, y, 7, 7),
            ];
        }
        Target::Origin => {
            distances = [
                |x, y| maze_calc_distance(x, y, 0, 0),
                |_x, _y| u8::MAX,
                |_x, _y| u8::MAX,
                |_x, _y| u8::MAX,
            ];
        }
    }

    for x in 0..MAZE_WIDTH_U8 {
        for y in 0..MAZE_HEIGHT_U8 {
            points[xy_to_index(x, y)] = Segment {
                pos: Vecu { x, y },
                distance: *distances.map(|f| f(x, y)).iter().min().unwrap(),
                walls: walls(x, y),
            };
        }
    }

    Maze { segments: points }
}

impl Maze {

    /// Creates a new maze centered around the center.
    pub fn new() -> Self {
        with_walls_fn(Target::Center, |_x, _y| [false, false, false, false])
    }

    /// Creates a new maze with the specified target and the existing maze to use walls from.
    pub fn with_walls(target: Target, maze: Maze) -> Self {
        with_walls_fn(target, |x, y| maze.segment(x, y).walls)
    }

    /// Returns the segment at `x, y`.
    pub fn segment(&self, x: u8, y: u8) -> Segment {
        self.segments[xy_to_index(x, y)]
    }

    /// Returns the segment at `x, y`.
    pub fn segment_vec(&self, pos: Vecu) -> Segment {
        self.segment(pos.x, pos.y)
    }

    /// Adds a wall at the specified direction.
    fn _add_wall(&mut self, x: u8, y: u8, direction: Relative) {
        let i = xy_to_index(x, y);
        let mut existing = self.segments[i];
        existing.walls[direction as usize] = true;
        self.segments[i] = existing;
    }

    /// Updates the walls of the segment at `x, y` to the specified array.
    pub fn update_walls(&mut self, x: u8, y: u8, walls: [bool; 4]) {
        let i = xy_to_index(x, y);
        let mut existing = self.segments[i];

        for (i, val) in existing.walls.iter().enumerate() {
            assert!(
                (!(*val) && !walls[i]) || (!(*val) && walls[i]) || (*val && walls[i]),
                "Walls can only be updated from false to true"
            )
        }

        existing.walls = walls;
        self.segments[i] = existing;

        // update neighbouring segments' walls
        for (j, dir) in Relative::iter().enumerate() {
            if !walls[j] {
                continue;
            }

            let relative = existing.relative(self, dir);
            if relative.is_none() {
                continue;
            }

            let mut relative = relative.unwrap();
            relative.walls[dir.opposite() as usize] = true;
            self.segments[pos_to_index(relative.pos())] = relative;
        }
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

/// The relative direction.
#[derive(PartialEq, Debug)]
pub enum Relative {
    North,
    East,
    South,
    West,
}

impl Relative {
    /// Returns an iterator over all relative directions.
    pub fn iter() -> Iter<'static, Relative> {
        static DIRECTIONS: [Relative; 4] = [
            Relative::North,
            Relative::East,
            Relative::South,
            Relative::West,
        ];
        DIRECTIONS.iter()
    }

    /// Returns the direction opposite to this direction.
    pub fn opposite(&self) -> Relative {
        match self {
            Relative::North => Relative::South,
            Relative::East => Relative::West,
            Relative::South => Relative::North,
            Relative::West => Relative::East,
        }
    }
}

/// Represents a point on the grid.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Segment {
    pos: Vecu,
    pub distance: u8,
    pub walls: [bool; 4],
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

    /// Creates a new default Segment.
    pub fn with_pos(pos: Vecu) -> Self {
        Segment {
            pos,
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

#[cfg(test)]
mod tests {
    use crate::maze::Maze;
    use std::panic::{catch_unwind, AssertUnwindSafe};

    #[test]
    fn test_wall_update_panics() {
        let mut maze = Maze::new();
        maze.update_walls(0, 0, [false, true, false, false]);
        assert!(catch_unwind(AssertUnwindSafe(|| maze.update_walls(
            0,
            0,
            [false, false, false, false]
        )))
        .is_err())
    }

    #[test]
    fn test_wall_update() {
        let mut maze = Maze::new();
        maze.update_walls(0, 0, [false, true, false, false]);

        assert_eq!([false, true, false, false], maze.segment(0, 0).walls);
        assert_eq!([false, false, false, true], maze.segment(1, 0).walls);

        let mut maze = Maze::new();
        maze.update_walls(0, 0, [false, false, true, false]);

        assert_eq!([false, false, true, false], maze.segment(0, 0).walls);
        assert_eq!([true, false, false, false], maze.segment(0, 1).walls);

        let mut maze = Maze::new();
        maze.update_walls(1, 0, [false, false, false, true]);

        assert_eq!([false, false, false, true], maze.segment(1, 0).walls);
        assert_eq!([false, true, false, false], maze.segment(0, 0).walls);
    }
}
