use crate::maze::Segment;
use crate::vec::Veci;
use crate::MAZE_SIZE;
use std::fmt;

/// Represents a path that may be taken
pub struct Path {
    /// The taken segments
    segments: Vec<Veci>,
}

impl Path {
    /// Returns a new path instance
    pub fn new() -> Self {
        Path {
            segments: Vec::with_capacity(MAZE_SIZE),
        }
    }

    /// Returns the current size of this path.
    pub fn size(&self) -> usize {
        self.segments.len()
    }

    /// Return the _n_-th segment that this path has taken.
    /// If the segment has not been visited yet, returns [Segment::new].
    pub fn segment(&self, index: usize) -> Option<Veci> {
        if index >= self.size() {
            return None
        }
        Some(self.segments[index])
    }

    /// Returns the current head of the path.
    pub fn head(&self) -> Option<Veci> {
        if self.size() == 0 {
            return None
        }
        Some(self.segments[self.size() - 1])
    }

    /// Appends a segment to the path.
    ///
    /// ### Arguments
    ///
    /// - `segment` - The `Segment` to append to the path.
    pub fn append(&mut self, segment: Veci) {
        self.segments.push(segment);
    }

    /// Appends segments to the path.
    ///
    /// ### Arguments
    ///
    /// - `segments` - The `Segment`s to append to the path.
    pub fn append_all(&mut self, segments: Vec<Veci>) {
        for segment in segments.iter() {
            self.segments.push(*segment)
        }
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
        assert_eq!(Segment::new().pos(), path.segment(0).expect("Failed to find first path segment"));
    }
}
