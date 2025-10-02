use crate::maze::{Maze, Relative, Segment};
use crate::path::Path;
use crate::vec::Veci;

/// The result of an attempted pathfinding using [next].
pub enum Result {

    /// Indicates that a suboptimal path has been found and that
    /// the mouse cannot follow strictly decreasing segments.
    Stuck,

    /// Indicates that a valid next segment has been found.
    Found(Segment)

}

impl Result {

    /// Whether this result is a dead end or not.
    pub fn is_dead_end(&self) -> bool {
        matches!(*self, Result::Stuck)
    }

    /// Whether this result contains a found [Segment] or not.
    pub fn is_found(&self) -> bool {
        matches!(*self, Result::Found(_))
    }

    /// Unwraps this result to retrieve a [Segment].
    pub fn unwrap(self) -> Segment {
        match self {
            Result::Found(val) => val,
            Result::Stuck => panic!("Called `Result::unwrap()` on no value"),
        }
    }

}

/// Attempts to find the next segment based on `maze` and the taken `path`.
///
/// ### Arguments
///
/// - `maze` - The current maze.
/// - `path` - The taken path.
///
/// ### Returns
///
/// - [Result::Stuck] - The path has reached a local minimum.
/// - [Result::Found] - A valid next segment has been found.
pub fn next(maze: &Maze, path: &Path) -> Result {
    // the smallest segment so far
    let mut min_segment = maze.segment_vec(path.head().unwrap_or_else(|| Veci::new()));
    // the biggest distance
    let max_distance = min_segment.distance;

    for i in 0..path.size() {
        let current = maze.segment_vec(path.segment(i).expect("Failed to find path segment"));

        for (i, dir) in Relative::iterator().enumerate() {
            if current.walls[i] {
                continue;
            }

            let segment = current.relative(maze, dir);

            if segment.is_none() {
                continue;
            }

            let segment = segment.unwrap();

            if segment.distance < min_segment.distance {
                min_segment = segment;
            }
        }
    }

    if min_segment.distance == max_distance {
        Result::Stuck
    } else {
        Result::Found(min_segment)
    }
}

/// Returns a path to the unvisited segment location relative to the head of `path`.
/// Does not include the head of `path`.
///
/// ### Arguments
///
/// - `maze` - The current maze.
/// - `path` - The taken path.
///
/// ### Returns
///
/// A [Vec] with all segment locations that should be followed to the nearest unvisited
/// segment.
pub fn next_unvisited(maze: &Maze, path: &Path) -> Vec<Veci> {
    let mut vec = Vec::with_capacity(12);

    for i in (0..path.size()).rev() {
        let veci = path.segment(i).expect("Failed to find path segment");
        let current = maze.segment_vec(veci);
        if i < path.size() - 1 {
            vec.push(veci);
        }

        for (i, dir) in Relative::iterator().enumerate() {
            if current.walls[i] {
                continue;
            }

            let segment = current.relative(maze, dir);

            if segment.is_none() {
                continue;
            }

            let segment = segment.unwrap();

            let mut visited = false;
            for i in (0..path.size()).rev() {
                if segment.pos() == path.segment(i).expect("Failed to find path segment") {
                    visited = true;
                }
            }

            if !visited {
                vec.push(segment.pos());
                return vec;
            }
        }
    }

    vec
}

#[cfg(test)]
mod tests {
    use crate::maze::Maze;
    use crate::path::Path;
    use crate::pathfinder;
    use crate::vec::Veci;

    /// Finds any segment that has a distance of zero.
    /// Updates `path` on the way.
    ///
    /// ### Arguments
    ///
    /// - `maze` - The maze.
    /// - `path` - The path that has been taken so far. Is updated by this method.
    fn find(maze: &mut Maze, path: &mut Path)  {
        path.append(Veci::new());

        loop {
            let result = pathfinder::next(&maze, &path);

            if result.is_found() {
                let next = result.unwrap();
                path.append(next.pos());

                if next.distance == 0 {
                    break;
                }
            } else {
                unimplemented!("{:?}", format_args!("DeadEnd unimplemented at {:?}", path.head()))
            }
        }
    }

    #[test]
    fn next() {
        let mut maze = Maze::new();
        let mut path = Path::new();

        find(&mut maze, &mut path);

        assert_eq!(15, path.size());
        assert_eq!(Veci { x: 0, y: 0 }, path.segment(0).expect("Failed to find path segment"));
        assert_eq!(Veci { x: 1, y: 0 }, path.segment(1).expect("Failed to find path segment"));
        assert_eq!(Veci { x: 2, y: 0 }, path.segment(2).expect("Failed to find path segment"));
        assert_eq!(Veci { x: 3, y: 0 }, path.segment(3).expect("Failed to find path segment"));
        assert_eq!(Veci { x: 4, y: 0 }, path.segment(4).expect("Failed to find path segment"));
        assert_eq!(Veci { x: 5, y: 0 }, path.segment(5).expect("Failed to find path segment"));
        assert_eq!(Veci { x: 6, y: 0 }, path.segment(6).expect("Failed to find path segment"));
        assert_eq!(Veci { x: 7, y: 0 }, path.segment(7).expect("Failed to find path segment"));
        assert_eq!(Veci { x: 7, y: 1 }, path.segment(8).expect("Failed to find path segment"));
        assert_eq!(Veci { x: 7, y: 2 }, path.segment(9).expect("Failed to find path segment"));
        assert_eq!(Veci { x: 7, y: 3 }, path.segment(10).expect("Failed to find path segment"));
        assert_eq!(Veci { x: 7, y: 4 }, path.segment(11).expect("Failed to find path segment"));
        assert_eq!(Veci { x: 7, y: 5 }, path.segment(12).expect("Failed to find path segment"));
        assert_eq!(Veci { x: 7, y: 6 }, path.segment(13).expect("Failed to find path segment"));
        assert_eq!(Veci { x: 7, y: 7 }, path.segment(14).expect("Failed to find path segment"));
        assert_eq!(Veci { x: 0, y: 0 }, path.segment(15).expect("Failed to find path segment"));
    }

    #[test]
    fn next_guided() {
        let mut maze = Maze::new();
        let mut path = Path::new();

        // ###
        // # #
        // # #
        maze.update_walls(0, 0, [true, true, false, true]);
        maze.update_walls(0, 1, [false, true, false, true]);

        find(&mut maze, &mut path);

        assert_eq!(Veci { x: 0, y: 0 }, path.segment(0).expect("Failed to find path segment"));
        assert_eq!(Veci { x: 0, y: 1 }, path.segment(1).expect("Failed to find path segment"));
        assert_eq!(Veci { x: 0, y: 2 }, path.segment(2).expect("Failed to find path segment"));
        assert_eq!(Veci { x: 1, y: 2 }, path.segment(3).expect("Failed to find path segment"));
        assert_eq!(Veci { x: 2, y: 2 }, path.segment(4).expect("Failed to find path segment"));
    }

    #[test]
    fn next_guided_diagonal() {
        let mut maze = Maze::new();
        let mut path = Path::new();

        // ######
        // #  ###
        // ##  ##
        // ###  #
        maze.update_walls(0, 0, [true, false, true, true]);
        maze.update_walls(1, 0, [true, true, false, false]);
        maze.update_walls(1, 1, [false, false, true, true]);
        maze.update_walls(2, 1, [true, true, false, false]);
        maze.update_walls(2, 2, [false, false, true, true]);
        maze.update_walls(3, 2, [true, true, false, false]);

        find(&mut maze, &mut path);

        assert_eq!(Veci { x: 0, y: 0 }, path.segment(0).expect("Failed to find path segment"));
        assert_eq!(Veci { x: 1, y: 0 }, path.segment(1).expect("Failed to find path segment"));
        assert_eq!(Veci { x: 1, y: 1 }, path.segment(2).expect("Failed to find path segment"));
        assert_eq!(Veci { x: 2, y: 1 }, path.segment(3).expect("Failed to find path segment"));
        assert_eq!(Veci { x: 2, y: 2 }, path.segment(4).expect("Failed to find path segment"));
        assert_eq!(Veci { x: 3, y: 2 }, path.segment(5).expect("Failed to find path segment"));
    }

    #[test]
    fn next_branch() {
        let mut maze = Maze::new();
        let mut path = Path::new();

        // #####
        // #   #
        // ## ##
        maze.update_walls(0, 0, [true, false, true, true]);
        maze.update_walls(1, 0, [true, false, false, false]);
        maze.update_walls(2, 0, [true, true, true, false]);
        maze.update_walls(1, 1, [false, false, false, false]);

        find(&mut maze, &mut path);

        assert_eq!(Veci { x: 0, y: 0 }, path.segment(0).expect("Failed to find path segment"));
        assert_eq!(Veci { x: 1, y: 0 }, path.segment(1).expect("Failed to find path segment"));
        assert_eq!(Veci { x: 2, y: 0 }, path.segment(2).expect("Failed to find path segment"));
        assert_eq!(Veci { x: 1, y: 0 }, path.segment(3).expect("Failed to find path segment"));
        assert_eq!(Veci { x: 1, y: 1 }, path.segment(4).expect("Failed to find path segment"));
    }
}
