use crate::vec2::Vec2i;
use crate::{MAZE_HEIGHT, MAZE_SIZE, MAZE_WIDTH};
use std::fmt;

/// Represents the maze
pub struct Maze {
    points: [Segment; MAZE_SIZE],
}

/// Calculates the distance to the specified point
fn maze_calc_distance(x: u16, y: u16, cx: i8, cy: i8) -> u8 {
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
                    pos: Vec2i { x, y },
                    distance: *distances.iter().min().unwrap(),
                };
            }
        }

        Maze { points }
    }

    /// Returns all the points in this maze.
    pub fn points(&self) -> [Segment; MAZE_SIZE] {
        self.points
    }
}

impl fmt::Debug for Maze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, segment) in self.points.iter().enumerate() {
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
#[derive(Copy, Clone, Debug)]
pub struct Segment {
    pos: Vec2i,
    pub distance: u8,
}

impl Segment {
    /// Creates a new default Segment.
    fn new() -> Self {
        Segment {
            pos: Vec2i { x: 0, y: 0 },
            distance: 0,
        }
    }

    /// Returns the position of this segment.
    fn pos(&self) -> Vec2i {
        self.pos
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
            " 7  6  5  4  3  2  1  0  0  1  2  3  4  5  6  7 "
        ];

        for &line in &expected {
            assert!(format!("{:?}", maze).lines().any(|l| l == line));
        }
    }
}
