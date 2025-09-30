use std::fmt;
use crate::maze::{Maze, Segment};
use crate::{MAZE_SIZE, MAZE_WIDTH};

/// Represents a path that may be taken
pub struct Path {
    /// The current size of the path
    size: usize,

    /// The taken segments
    segments: [Segment; MAZE_SIZE]
}

impl Path {

    /// Returns a new path instance
    pub fn new() -> Self {
        Path {
            size: 0,
            segments: [Segment::new(); MAZE_SIZE]
        }
    }

    /// Returns the current size of this path.
    pub fn size(&self) -> usize {
        self.size
    }

    /// Return all segments that this path has taken.
    pub fn segments(&self) -> [Segment; MAZE_SIZE] {
        self.segments
    }

    /// Returns the current head of the path.
    pub fn head(&self) -> Segment {
        self.segments[self.size]
    }

    /// Appends a segment to the path.
    ///
    /// ### Arguments
    ///
    /// * `segment` - The `Segment` to append to the path.
    pub fn append(&mut self, segment: Segment) {
        self.segments[self.size] = segment;
        self.size += 1;
    }

}

impl fmt::Debug for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for segment in self.segments.iter() {
            write!(f, "{:?} -> ", segment.pos())?;
        }
        Ok(())
    }
}
