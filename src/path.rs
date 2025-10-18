use crate::map::Map;
use crate::vec::Vecu;
use crate::{MAZE_HEIGHT_U8, MAZE_SIZE, MAZE_WIDTH_U8};
use heapless::Vec;

pub const ACCELERATION_MS2: f64 = 2.;
pub const MAX_SPEED_MS: f64 = 5.;

/// Represents a path that may be taken
pub struct Path {
    /// The taken segments
    segments: Vec<Vecu, MAZE_SIZE>,
    optimized: bool,
}

impl Path {
    /// Returns a new path instance
    pub fn new() -> Self {
        Path {
            segments: Vec::new(),
            optimized: false,
        }
    }

    /// Whether the current value at a path is a turn or not.
    fn is_turn(&self, current: usize) -> bool {
        let prev = self.segments[current - 1];
        let next = self.segments[current + 1];

        if prev.x != next.x && prev.y != next.y {
            true
        } else {
            false
        }
    }

    /// Returns the current estimated amount of time to complete this path.
    /// Returns undefined results on unoptimized paths.
    pub fn time_to_complete(&self) -> f64 {
        assert!(
            self.optimized,
            "cannot calculate time to complete on an unoptimized path"
        );

        let mut time = 0.;

        for (i, _x) in self.segments.iter().enumerate() {
            if i > 0 && i < self.segments.len() - 1 {
                if self.is_turn(i) {
                    time += 0.5;
                }
                time += 1.;
            }
        }

        time
    }

    /// Returns the current size of this path.
    pub fn len(&self) -> usize {
        self.segments.len()
    }

    /// Return the _n_-th segment that this path has taken.
    /// If the segment has not been visited yet, returns [Segment::new].
    pub fn segment(&self, index: usize) -> Option<Vecu> {
        if index >= self.len() {
            return None;
        }
        Some(self.segments[index])
    }

    /// Returns the current head of the path.
    pub fn head(&self) -> Option<Vecu> {
        if self.len() == 0 {
            return None;
        }
        Some(self.segments[self.len() - 1])
    }

    /// Appends a segment to the path.
    ///
    /// ### Arguments
    ///
    /// - `segment` - The `Segment` to append to the path.
    pub fn append(&mut self, segment: Vecu) {
        self.segments.push(segment).unwrap();
    }

    /// Appends segments to the path.
    ///
    /// ### Arguments
    ///
    /// - `segments` - The `Segment`s to append to the path.
    pub fn append_all(&mut self, segments: &[Vecu]) {
        for segment in segments.into_iter() {
            self.segments.push(*segment).unwrap();
        }
    }

    /// Returns whether this path contains the specified vector.
    ///
    /// ### Arguments
    ///
    /// - `vec` - The vec to check for containment.
    pub fn contains(&self, vec: Vecu) -> bool {
        for i in (0..self.len()).rev() {
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
        let mut optimized: Vec<Vecu, MAZE_SIZE> = Vec::new();

        let mut i = 0;
        'outer: while i < self.segments.len() {
            let pos = self.segments[i];

            for j in i + 1..self.segments.len() {
                if self.segments[j] != pos {
                    continue;
                }

                i += j - i;
                continue 'outer;
            }

            optimized.push(pos).unwrap();
            i += 1;
        }

        self.segments = optimized;
        self.optimized = true;
    }

    /// Returns whether this path has been optimized.
    pub fn optimized(&self) -> bool {
        self.optimized
    }
}

impl core::fmt::Debug for Path {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Path | Length {:?} | Time to complete {:?} sec\n",
            self.segments.len(),
            if self.optimized {
                self.time_to_complete()
            } else {
                0.
            }
        )?;

        for y in 0..MAZE_HEIGHT_U8 {
            for x in 0..MAZE_WIDTH_U8 {
                if self.contains(Vecu { x, y }) {
                    write!(f, "#")?;
                } else {
                    write!(f, " ")?;
                }
            }
            write!(f, "\n")?;
        }
        write!(f, "\n")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::maze::Segment;
    use crate::path::Path;
    use crate::vec::Vecu;

    #[test]
    fn path() {
        let mut path = Path::new();

        path.append(Vecu::new());
        assert_eq!(1, path.len());
        assert_eq!(
            Segment::new().pos(),
            path.segment(0).expect("Failed to find first path segment")
        );
    }

    #[test]
    fn optimize() {
        let mut path = Path::new();

        path.append(Vecu { x: 0, y: 0 });
        path.append(Vecu { x: 1, y: 0 });
        path.append(Vecu { x: 0, y: 1 });
        path.append(Vecu { x: 1, y: 0 });
        path.append(Vecu { x: 3, y: 1 });
        path.append(Vecu { x: 1, y: 0 });
        path.append(Vecu { x: 2, y: 0 });

        path.optimize();

        assert_eq!(3, path.len());
        assert_eq!(Vecu { x: 0, y: 0 }, path.segment(0).unwrap());
        assert_eq!(Vecu { x: 1, y: 0 }, path.segment(1).unwrap());
        assert_eq!(Vecu { x: 2, y: 0 }, path.segment(2).unwrap());
    }

    #[test]
    fn optimize_avoid_remove_root() {
        let mut path = Path::new();

        path.append(Vecu { x: 0, y: 0 });
        path.append(Vecu { x: 1, y: 0 });
        path.append(Vecu { x: 2, y: 0 });
        path.append(Vecu { x: 3, y: 0 });
        path.append(Vecu { x: 4, y: 0 });
        path.append(Vecu { x: 3, y: 0 });
        path.append(Vecu { x: 2, y: 0 });
        path.append(Vecu { x: 1, y: 0 });
        path.append(Vecu { x: 1, y: 1 });
        path.optimize();

        // assert_eq!(3, path.len());
        assert_eq!(Vecu { x: 0, y: 0 }, path.segment(0).unwrap());
        assert_eq!(Vecu { x: 1, y: 0 }, path.segment(1).unwrap());
        assert_eq!(Vecu { x: 1, y: 1 }, path.segment(2).unwrap());
    }


    #[test]
    fn turns() {
        let mut one = Path::new();

        one.append(Vecu { x: 0, y: 0 });
        one.append(Vecu { x: 1, y: 0 });
        one.append(Vecu { x: 2, y: 0 });
        one.append(Vecu { x: 3, y: 0 });
        one.append(Vecu { x: 3, y: 1 });
        one.append(Vecu { x: 3, y: 2 });
        one.append(Vecu { x: 3, y: 3 });

        let mut two = Path::new();

        two.append(Vecu { x: 0, y: 0 });
        two.append(Vecu { x: 1, y: 0 });
        two.append(Vecu { x: 2, y: 0 });
        two.append(Vecu { x: 3, y: 0 });
        two.append(Vecu { x: 4, y: 0 });
        two.append(Vecu { x: 4, y: 1 });
        two.append(Vecu { x: 4, y: 2 });
        two.append(Vecu { x: 4, y: 3 });
        two.append(Vecu { x: 3, y: 3 });

        one.optimize();
        two.optimize();

        assert!(one.time_to_complete() < two.time_to_complete());
    }

    #[test]
    fn turns_equal_len() {
        let mut one = Path::new();

        one.append(Vecu { x: 0, y: 0 });
        one.append(Vecu { x: 1, y: 0 });
        one.append(Vecu { x: 2, y: 0 });
        one.append(Vecu { x: 3, y: 0 });
        one.append(Vecu { x: 3, y: 1 });
        one.append(Vecu { x: 3, y: 2 });
        one.append(Vecu { x: 3, y: 3 });
        one.append(Vecu { x: 2, y: 3 });
        one.append(Vecu { x: 1, y: 3 });
        one.append(Vecu { x: 0, y: 3 });

        let mut two = Path::new();

        two.append(Vecu { x: 0, y: 0 });
        two.append(Vecu { x: 0, y: 1 });
        two.append(Vecu { x: 1, y: 1 });
        two.append(Vecu { x: 2, y: 1 });
        two.append(Vecu { x: 3, y: 1 });
        two.append(Vecu { x: 3, y: 2 });
        two.append(Vecu { x: 3, y: 3 });
        two.append(Vecu { x: 2, y: 3 });
        two.append(Vecu { x: 1, y: 3 });
        two.append(Vecu { x: 0, y: 3 });

        one.optimize();
        two.optimize();

        assert!(one.time_to_complete() < two.time_to_complete());
    }
}
