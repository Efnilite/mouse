use crate::vec::Veci;
use crate::{MAZE_HEIGHT, MAZE_SIZE, MAZE_WIDTH};
use std::fmt;

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

        for x in 0..MAZE_WIDTH {
            for y in 0..MAZE_HEIGHT {
                let distances = [
                    maze_calc_distance(x, y, 8, 8),
                    maze_calc_distance(x, y, 7, 8),
                    maze_calc_distance(x, y, 8, 7),
                    maze_calc_distance(x, y, 7, 7),
                ];

                points[(x + y * MAZE_WIDTH) as usize] = Segment {
                    pos: Veci { x, y },
                    distance: *distances.iter().min().unwrap(),
                    walls: [false, false, false, false],
                };
            }
        }

        Maze { segments: points }
    }

    /// Returns the point at `x, y`.
    pub fn point(&self, x: u8, y: u8) -> Segment {
        self.segments[(x + y * MAZE_WIDTH) as usize]
    }
}

impl fmt::Debug for Maze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, segment) in self.segments.iter().enumerate() {
            if segment.distance < 10 {
                write!(f, " {:?} ", segment.distance)?;
            } else {
                write!(f, "{:?} ", segment.distance)?;
            }
            if (i + 1) % MAZE_WIDTH as usize == 0 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

/// Represents a point on the grid.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Segment {
    pos: Veci,
    pub distance: u8,
    pub walls: [bool; 4],
}

/// The relative direction.
#[derive(PartialEq)]
pub enum Relative {
    NORTH,
    WEST,
    SOUTH,
    EAST,
}

impl Segment {
    /// Creates a new default Segment.
    pub fn new() -> Self {
        Segment {
            pos: Veci { x: 0, y: 0 },
            distance: u8::MAX,
            walls: [false, false, false, false],
        }
    }

    /// Returns the position of this segment.
    pub fn pos(&self) -> Veci {
        self.pos
    }

    /// Returns the segment relative to this one by direction `relative`.
    ///
    /// ### Arguments
    ///
    /// - `maze` - A maze ref.
    /// - `relative` - The direction.
    pub fn relative(&self, maze: &Maze, relative: Relative) -> Result<Segment, u8> {
        if relative == Relative::SOUTH && self.pos.y > MAZE_HEIGHT - 1 {
            return Err(0);
        } else if relative == Relative::NORTH && self.pos.y == 0 {
            return Err(0);
        } else if relative == Relative::EAST && self.pos.x > MAZE_WIDTH - 1 {
            return Err(0);
        } else if relative == Relative::WEST && self.pos.x == 0 {
            return Err(0);
        }

        Ok(match relative {
            Relative::NORTH => maze.point(self.pos.x, self.pos.y - 1),
            Relative::WEST => maze.point(self.pos.x - 1, self.pos.y),
            Relative::SOUTH => maze.point(self.pos.x, self.pos.y + 1),
            Relative::EAST => maze.point(self.pos.x + 1, self.pos.y),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::maze::Maze;

    #[test]
    fn maze_new() {
        let maze = Maze::new();
        let expected = [
            "14 13 12 11 10  9  8  7  7  8  9 10 11 12 13 14 ",
            "13 12 11 10  9  8  7  6  6  7  8  9 10 11 12 13 ",
            "12 11 10  9  8  7  6  5  5  6  7  8  9 10 11 12 ",
            "11 10  9  8  7  6  5  4  4  5  6  7  8  9 10 11 ",
            "11 10  9  8  7  6  5  4  4  5  6  7  8  9 10 11 ",
            " 8  7  6  5  4  3  2  1  1  2  3  4  5  6  7  8 ",
            " 7  6  5  4  3  2  1  0  0  1  2  3  4  5  6  7 ",
        ];

        for &line in &expected {
            assert!(format!("{:?}", maze).lines().any(|l| l == line));
        }
    }
}
