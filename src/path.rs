use crate::maze::Segment;
use crate::vec::Veci;
use crate::MAZE_SIZE;
use std::fmt;

/// Represents a path that may be taken
pub struct Path {
    /// The current size of the path
    size: usize,

    /// The taken segments
    segments: [Veci; MAZE_SIZE],
}

impl Path {
    /// Returns a new path instance
    pub fn new() -> Self {
        Path {
            size: 1,
            segments: [Veci::new(); MAZE_SIZE],
        }
    }

    /// Returns the current size of this path.
    pub fn size(&self) -> usize {
        self.size
    }

    /// Return the _n_-th segment that this path has taken.
    /// If the segment has not been visited yet, returns [Segment::new].
    pub fn segment(&self, index: usize) -> Veci {
        self.segments[index]
    }

    /// Returns the current head of the path.
    pub fn head(&self) -> Veci {
        self.segments[self.size - 1]
    }

    /// Appends a segment to the path.
    ///
    /// ### Arguments
    ///
    /// - `segment` - The `Segment` to append to the path.
    pub fn append(&mut self, segment: Veci) {
        self.segments[self.size] = segment;
        self.size += 1;
    }
}

impl fmt::Debug for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for segment in self.segments.iter() {
            write!(f, "{:?} -> ", *segment)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::maze::Segment;
    use crate::path::Path;
    use crate::vec::Veci;

    #[test]
    fn path() {
        let mut path = Path::new();

        path.append(Veci::new());
        assert_eq!(2, path.size());
        assert_eq!(Segment::new().pos(), path.segment(0));
    }
}
