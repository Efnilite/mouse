use crate::MAZE_SIZE;
use crate::vec::Veci;
use std::collections::HashMap;
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
            return None;
        }
        Some(self.segments[index])
    }

    /// Returns the current head of the path.
    pub fn head(&self) -> Option<Veci> {
        if self.size() == 0 {
            return None;
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
        for segment in segments.into_iter() {
            self.segments.push(segment)
        }
    }

    /// Returns whether this path contains the specified vector.
    ///
    /// ### Arguments
    ///
    /// - `vec` - The vec to check for containment.
    pub fn contains(&self, vec: Veci) -> bool {
        for i in (0..self.size()).rev() {
            if self.segments[i] == vec {
                return true;
            }
        }
        false
    }

    /// Optimizes this path by removing any cycle that has the same start and end point.
    /// Assigns `self.segments` to a new optimized [Vec].
    /// todo! avoid bulk optimization and optimize as soon as append_all/append is called
    pub fn optimize(&mut self) {
        // the first found position of every veci
        let mut occurrences = HashMap::new();
        let mut optimized = Vec::with_capacity(self.size());

        for (i, pos) in self.segments.iter().enumerate() {
            let existing = occurrences.insert(*pos, i);

            if existing.is_none() {
                optimized.push(*pos);
                continue;
            }

            let existing = existing.unwrap();
            for _ in (existing..i).rev() {
                optimized.pop();
            }
            optimized.push(*pos);
        }

        self.segments = optimized;
    }
}

impl fmt::Debug for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, segment) in self.segments.iter().enumerate() {
            if i == self.size() - 1 {
                write!(f, "{:?}", *segment)?;
            } else {
                write!(f, "{:?} -> ", *segment)?;
            }
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
        assert_eq!(1, path.size());
        assert_eq!(
            Segment::new().pos(),
            path.segment(0).expect("Failed to find first path segment")
        );
    }

    #[test]
    fn optimize() {
        let mut path = Path::new();

        path.append(Veci { x: 0, y: 0});
        path.append(Veci { x: 1, y: 0});
        path.append(Veci { x: 0, y: 1});
        path.append(Veci { x: 1, y: 0});
        path.append(Veci { x: 3, y: 1});
        path.append(Veci { x: 1, y: 0});
        path.append(Veci { x: 2, y: 0});

        path.optimize();

        assert_eq!(3, path.size());
        assert_eq!(Veci { x: 0, y: 0 },  path.segment(0).unwrap());
        assert_eq!(Veci { x: 1, y: 0 },  path.segment(1).unwrap());
        assert_eq!(Veci { x: 2, y: 0 },  path.segment(2).unwrap());
    }
}
