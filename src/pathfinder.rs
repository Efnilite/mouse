use crate::map::Map;
use crate::maze::{Maze, Relative, Segment};
use crate::path::Path;
use crate::vec::Vecu;
use crate::MAZE_SIZE;
use heapless::{Deque, Vec};

/// The result of an attempted pathfinding using [next].
pub enum Result {
    /// Indicates that a segment has been found that is not a neighbour of the head of the path.
    /// Contains the new segment and the path from the head to the segment.
    Stuck(Vec<Vecu, MAZE_SIZE>),

    /// Indicates that a valid neighbour has been found as the next segment.
    Found(Segment),
}

impl Result {

    /// Whether this result is a dead end or not.
    pub fn is_dead_end(&self) -> bool {
        matches!(*self, Result::Stuck(_))
    }

    /// Whether this result contains a found [Segment] or not.
    pub fn is_found(&self) -> bool {
        matches!(*self, Result::Found(_))
    }

    /// Unwraps the found value.
    pub fn unwrap_found(self) -> Segment {
        match self {
            Result::Found(s) => s,
            _ => panic!("Called `Result::unwrap_found` on a non-Found value"),
        }
    }

    /// Unwraps the stuck value.
    pub fn unwrap_stuck(&self) -> &Vec<Vecu, MAZE_SIZE> {
        match self {
            Result::Stuck(s) => s,
            _ => panic!("Called `Result::unwrap_stuck` on a non-Stuck value"),
        }
    }

}

/// Attempts to find the next segment based on `maze` and the taken `path`.
///
/// ### Description
///
/// Returns the segment `n` with the smallest non-strict distance to the target
/// that is reachable from and a neighbour of `p`. `p` is any value in `path`.
///
/// ### Implementation
///
/// For every element in path, starting from the beginning, all neighbours are checked.
/// If a neighbour `n` is reachable and has not yet been visited, it is considered as a potential
/// minimal segment. If `n` has a lower distance than the current minimal segment, `n` becomes the
/// minimal segment.
///
/// If the minimal segment is disconnected from the head, returns a path to the minimal segment.
/// This is not guaranteed to be the shortest path.
///
/// ### Arguments
///
/// - `maze` - The current maze.
/// - `path` - The taken path.
///
/// ### Returns
///
/// - [Result::Found] - A valid next segment has been found.
/// - [Result::Stuck] - A valid next segment has been found, but it is not directly attached
/// to the head of `path`. Returns the path to the valid next segment.
pub fn next(maze: &Maze, path: &Path) -> Result {
    // the smallest segment so far
    let mut min_segment = Segment::new();
    // the distance from min segment to the current segment in the loop
    let mut min_segment_distance = 0usize;

    for i in (0..path.len()).rev() {
        let current = maze.segment_vec(path.segment(i)
            .expect("Failed to find path segment"));

        'dirs: for (i, dir) in Relative::iter().enumerate() {
            if current.walls[i] {
                continue 'dirs;
            }

            let segment = current.relative(maze, dir);

            if segment.is_none() {
                continue 'dirs;
            }

            let segment = segment.unwrap();

            if path.contains(segment.pos()) {
                continue 'dirs;
            }

            if segment.distance < min_segment.distance {
                min_segment = segment;
                min_segment_distance = path.len() - i;
            }
        }
    }

    if maze.segment_vec(path.head().unwrap()).distance <= min_segment.distance {
        let mut to_min: Vec<Vecu, MAZE_SIZE> = Vec::new();
        for i in (0..min_segment_distance).rev() {
            to_min.push(path.segment(i).unwrap()).unwrap();
        }

        Result::Stuck(to_min)
    } else {
        Result::Found(min_segment)
    }
}

#[derive(Debug)]
struct ExploredNode {
    /// The parent of the explored node, null if it is the root.
    parent: Option<Vecu>,
}

/// Returns a path to the closest unvisited segment, relative to the head of `path`.
/// The segments are ordered from the current head of `path` to the target segment.
/// Includes the head of `path`.
///
/// ### Description
///
/// Finds the closest unvisited segment `t` to the current head of `path`.
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
pub fn next_unvisited(maze: &Maze, path: &Path) -> Vec<Vecu, MAZE_SIZE> {
    let mut to_explore: Deque<Vecu, MAZE_SIZE> = Deque::new();
    // contains the vecs that have been explored, with the value being the parent vec.
    // for the root, value is `None`.
    let mut explored: Map<ExploredNode> = Map::new();

    {
        let root = path.head().unwrap_or_else(|| Vecu::new());
        explored.insert(
            root,
            ExploredNode {
                parent: None,
            },
        );
        to_explore.push_back(root).unwrap();
    }

    while !to_explore.is_empty() {
        let current_pos = to_explore
            .pop_front()
            .expect("Failed to find unvisited node in entire maze");
        let current_segment = maze.segment_vec(current_pos);

        // found target
        if !path.contains(current_pos) {
            let mut to_exit: Vec<Vecu, MAZE_SIZE> = Vec::new();

            let mut parent = Some(current_pos);
            while parent != None {
                let value = parent.unwrap();
                to_exit.push(value).unwrap();
                parent = explored.get(&value).unwrap().parent;
            }

            // update distances by traversing the path and adding to distance
            // as long as the path is decreasing in distance.
            // do this for all possible exits of the path

            to_exit.reverse(); // path is constructed the wrong way around
            return to_exit;
        }

        // check all directions for unvisited segments
        'dirs: for (i, dir) in Relative::iter().enumerate() {
            if current_segment.walls[i] {
                continue 'dirs;
            }

            let relative = current_segment.relative(maze, dir);
            if relative.is_none() { // wall or OOB
                continue 'dirs;
            }

            let segment = relative.unwrap();
            if explored.contains_key(&segment.pos()) { // already explored
                continue 'dirs;
            }

            explored.insert(
                segment.pos(),
                ExploredNode {
                    parent: Some(current_pos),
                },
            );
            to_explore.push_back(segment.pos()).unwrap();
        }
    }

    panic!("Failed to find unvisited node in entire maze")
}

fn update_distances(maze: &Maze, path: &Path) {

}

#[cfg(test)]
mod tests {
    use crate::maze::Maze;
    use crate::path::Path;
    use crate::pathfinder::{next_unvisited};
    use crate::vec::Vecu;

    /// Finds any segment that has a distance of zero.
    /// Updates `path` on the way.
    ///
    /// ### Arguments
    ///
    /// - `maze` - The maze.
    /// - `path` - The path that has been taken so far. Is updated by this method.
    fn find(maze: &Maze, path: &mut Path) {
        path.append(Vecu::new());

        // loop {
        //     let result = crate::pathfinder::next(&maze, &path);
        //
        //     if result.is_found() {
        //         let next = result.unwrap();
        //         path.append(next.pos());
        //
        //         if next.distance == 0 {
        //             break;
        //         }
        //         continue;
        //     }
        //
        //     let mut segments = next_unvisited(&maze, &path);
        //     segments.remove(0);
        //     path.append_all(segments);
        // }
    }

    #[test]
    fn next() {
        let mut maze = Maze::new();
        let mut path = Path::new();

        find(&mut maze, &mut path);

        assert_eq!(15, path.len());
        assert_eq!(Vecu { x: 0, y: 0 }, path.segment(0).unwrap());
        assert_eq!(Vecu { x: 1, y: 0 }, path.segment(1).unwrap());
        assert_eq!(Vecu { x: 2, y: 0 }, path.segment(2).unwrap());
        assert_eq!(Vecu { x: 3, y: 0 }, path.segment(3).unwrap());
        assert_eq!(Vecu { x: 4, y: 0 }, path.segment(4).unwrap());
        assert_eq!(Vecu { x: 5, y: 0 }, path.segment(5).unwrap());
        assert_eq!(Vecu { x: 6, y: 0 }, path.segment(6).unwrap());
        assert_eq!(Vecu { x: 7, y: 0 }, path.segment(7).unwrap());
        assert_eq!(Vecu { x: 7, y: 1 }, path.segment(8).unwrap());
        assert_eq!(Vecu { x: 7, y: 2 }, path.segment(9).unwrap());
        assert_eq!(Vecu { x: 7, y: 3 }, path.segment(10).unwrap());
        assert_eq!(Vecu { x: 7, y: 4 }, path.segment(11).unwrap());
        assert_eq!(Vecu { x: 7, y: 5 }, path.segment(12).unwrap());
        assert_eq!(Vecu { x: 7, y: 6 }, path.segment(13).unwrap());
        assert_eq!(Vecu { x: 7, y: 7 }, path.segment(14).unwrap());
        assert!(path.segment(15).is_none());
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

        assert_eq!(Vecu { x: 0, y: 0 }, path.segment(0).unwrap());
        assert_eq!(Vecu { x: 0, y: 1 }, path.segment(1).unwrap());
        assert_eq!(Vecu { x: 0, y: 2 }, path.segment(2).unwrap());
        assert_eq!(Vecu { x: 1, y: 2 }, path.segment(3).unwrap());
        assert_eq!(Vecu { x: 2, y: 2 }, path.segment(4).unwrap());
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

        assert_eq!(Vecu { x: 0, y: 0 }, path.segment(0).unwrap());
        assert_eq!(Vecu { x: 1, y: 0 }, path.segment(1).unwrap());
        assert_eq!(Vecu { x: 1, y: 1 }, path.segment(2).unwrap());
        assert_eq!(Vecu { x: 2, y: 1 }, path.segment(3).unwrap());
        assert_eq!(Vecu { x: 2, y: 2 }, path.segment(4).unwrap());
        assert_eq!(Vecu { x: 3, y: 2 }, path.segment(5).unwrap());
    }

    #[test]
    fn general_deadend() {
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

        assert_eq!(Vecu { x: 0, y: 0 }, path.segment(0).unwrap());
        assert_eq!(Vecu { x: 1, y: 0 }, path.segment(1).unwrap());
        assert_eq!(Vecu { x: 2, y: 0 }, path.segment(2).unwrap());
        assert_eq!(Vecu { x: 1, y: 0 }, path.segment(3).unwrap());
        assert_eq!(Vecu { x: 1, y: 1 }, path.segment(4).unwrap());

        assert_eq!(14, maze.segment(0, 0).distance);
        assert_eq!(13, maze.segment(1, 0).distance);
        assert_eq!(14, maze.segment(2, 0).distance);
        assert_eq!(12, maze.segment(1, 1).distance);
    }

    #[test]
    fn general_nontrivial_deadend() {
        let mut maze = Maze::new();
        let mut path = Path::new();

        // #######
        // #     #
        // # # # #
        // # #   #
        // # #####
        maze.update_walls(0, 0, [true, false, false, true]);
        maze.update_walls(1, 0, [true, false, false, false]);
        maze.update_walls(2, 0, [true, false, true, false]);
        maze.update_walls(3, 0, [true, true, false, false]);

        maze.update_walls(0, 1, [false, true, false, true]);
        maze.update_walls(1, 1, [false, false, true, true]);
        maze.update_walls(2, 1, [true, false, true, false]);
        maze.update_walls(3, 1, [false, true, true, false]);

        find(&mut maze, &mut path);

        assert_eq!(Vecu { x: 0, y: 0 }, path.segment(0).unwrap());
        assert_eq!(Vecu { x: 1, y: 0 }, path.segment(1).unwrap());
        assert_eq!(Vecu { x: 2, y: 0 }, path.segment(2).unwrap());
        assert_eq!(Vecu { x: 3, y: 0 }, path.segment(3).unwrap());
        assert_eq!(Vecu { x: 3, y: 1 }, path.segment(4).unwrap());
        assert_eq!(Vecu { x: 2, y: 1 }, path.segment(5).unwrap());
        assert_eq!(Vecu { x: 1, y: 1 }, path.segment(6).unwrap());
        assert_eq!(Vecu { x: 1, y: 0 }, path.segment(7).unwrap());
        assert_eq!(Vecu { x: 0, y: 0 }, path.segment(8).unwrap());
        assert_eq!(Vecu { x: 0, y: 1 }, path.segment(9).unwrap());
    }
}
