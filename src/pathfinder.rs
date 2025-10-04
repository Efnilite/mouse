use crate::map::Map;
use crate::maze::{Maze, Relative, Segment};
use crate::path::Path;
use crate::vec::Veci;
use crate::MAZE_SIZE;
use heapless::{Deque, Vec};

/// The result of an attempted pathfinding using [next].
pub enum Result {
    /// Indicates that a suboptimal path has been found and that
    /// the mouse cannot follow strictly decreasing segments.
    Stuck,

    /// Indicates that a valid next segment has been found.
    Found(Segment),
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
            }
        }
    }

    if min_segment.distance == max_distance {
        Result::Stuck
    } else {
        Result::Found(min_segment)
    }
}

#[derive(Debug)]
struct ExploredNode {
    /// The parent of the explored node, null if it is the root.
    parent: Option<Veci>,
    /// The distance to the root node.
    distance: u8,
}

/// Returns a path to the unvisited segment location relative to the head of `path`.
/// The segments are ordered from the current head of `path` to the target segment.
/// Includes the head of `path`.
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
pub fn next_unvisited(maze: &Maze, path: &Path) -> Vec<Veci, MAZE_SIZE> {
    let mut to_explore: Deque<Veci, MAZE_SIZE> = Deque::new();
    // contains the vecs that have been explored, with the value being the parent vec.
    // for the root, value is `None`.
    let mut explored: Map<ExploredNode> = Map::new();

    {
        let root = path.head().unwrap_or_else(|| Veci::new());
        explored.insert(
            root,
            ExploredNode {
                parent: None,
                distance: 0,
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
            let mut to_root: Vec<Veci, MAZE_SIZE> = Vec::new();

            let mut parent = Some(current_pos);
            while parent != None {
                let value = parent.unwrap();
                to_root.push(value).unwrap();
                parent = explored.get(&value).unwrap().parent;
            }

            // todo update distances

            to_root.reverse(); // path is constructed the wrong way around
            return to_root;
        }

        let parent_distance = explored.get(&current_pos).unwrap().distance;
        'dirs: for (i, dir) in Relative::iter().enumerate() {
            if current_segment.walls[i] {
                continue 'dirs;
            }

            let relative = current_segment.relative(maze, dir);
            if relative.is_none() {
                continue 'dirs;
            }

            let segment = relative.unwrap();
            if explored.contains_key(&segment.pos()) {
                continue;
            }

            explored.insert(
                segment.pos(),
                ExploredNode {
                    parent: Some(current_pos),
                    distance: parent_distance + 1,
                },
            );
            to_explore.push_back(segment.pos()).unwrap();
        }
    }

    panic!("Failed to find unvisited node in entire maze")
}

/// Finds any segment that has a distance of zero.
/// Updates `path` on the way.
///
/// ### Arguments
///
/// - `maze` - The maze.
/// - `path` - The path that has been taken so far. Is updated by this method.
fn find(maze: &Maze, path: &mut Path) {
    path.append(Veci::new());

    loop {
        let result = next(&maze, &path);

        if result.is_found() {
            let next = result.unwrap();
            path.append(next.pos());

            if next.distance == 0 {
                break;
            }
            continue;
        }

        let mut segments = next_unvisited(&maze, &path);
        segments.remove(0);
        path.append_all(segments);
    }
}

#[cfg(test)]
mod tests {
    use crate::maze::Maze;
    use crate::path::Path;
    use crate::pathfinder::find;
    use crate::vec::Veci;

    #[test]
    fn next() {
        let mut maze = Maze::new();
        let mut path = Path::new();

        find(&mut maze, &mut path);

        assert_eq!(15, path.size());
        assert_eq!(Veci { x: 0, y: 0 }, path.segment(0).unwrap());
        assert_eq!(Veci { x: 1, y: 0 }, path.segment(1).unwrap());
        assert_eq!(Veci { x: 2, y: 0 }, path.segment(2).unwrap());
        assert_eq!(Veci { x: 3, y: 0 }, path.segment(3).unwrap());
        assert_eq!(Veci { x: 4, y: 0 }, path.segment(4).unwrap());
        assert_eq!(Veci { x: 5, y: 0 }, path.segment(5).unwrap());
        assert_eq!(Veci { x: 6, y: 0 }, path.segment(6).unwrap());
        assert_eq!(Veci { x: 7, y: 0 }, path.segment(7).unwrap());
        assert_eq!(Veci { x: 7, y: 1 }, path.segment(8).unwrap());
        assert_eq!(Veci { x: 7, y: 2 }, path.segment(9).unwrap());
        assert_eq!(Veci { x: 7, y: 3 }, path.segment(10).unwrap());
        assert_eq!(Veci { x: 7, y: 4 }, path.segment(11).unwrap());
        assert_eq!(Veci { x: 7, y: 5 }, path.segment(12).unwrap());
        assert_eq!(Veci { x: 7, y: 6 }, path.segment(13).unwrap());
        assert_eq!(Veci { x: 7, y: 7 }, path.segment(14).unwrap());
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

        assert_eq!(Veci { x: 0, y: 0 }, path.segment(0).unwrap());
        assert_eq!(Veci { x: 0, y: 1 }, path.segment(1).unwrap());
        assert_eq!(Veci { x: 0, y: 2 }, path.segment(2).unwrap());
        assert_eq!(Veci { x: 1, y: 2 }, path.segment(3).unwrap());
        assert_eq!(Veci { x: 2, y: 2 }, path.segment(4).unwrap());
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

        assert_eq!(Veci { x: 0, y: 0 }, path.segment(0).unwrap());
        assert_eq!(Veci { x: 1, y: 0 }, path.segment(1).unwrap());
        assert_eq!(Veci { x: 1, y: 1 }, path.segment(2).unwrap());
        assert_eq!(Veci { x: 2, y: 1 }, path.segment(3).unwrap());
        assert_eq!(Veci { x: 2, y: 2 }, path.segment(4).unwrap());
        assert_eq!(Veci { x: 3, y: 2 }, path.segment(5).unwrap());
    }

    #[test]
    fn general_branch() {
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

        assert_eq!(Veci { x: 0, y: 0 }, path.segment(0).unwrap());
        assert_eq!(Veci { x: 1, y: 0 }, path.segment(1).unwrap());
        assert_eq!(Veci { x: 2, y: 0 }, path.segment(2).unwrap());
        assert_eq!(Veci { x: 1, y: 0 }, path.segment(3).unwrap());
        assert_eq!(Veci { x: 1, y: 1 }, path.segment(4).unwrap());
    }

    #[test]
    fn general_nontrivial_branch() {
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

        assert_eq!(Veci { x: 0, y: 0 }, path.segment(0).unwrap());
        assert_eq!(Veci { x: 1, y: 0 }, path.segment(1).unwrap());
        assert_eq!(Veci { x: 2, y: 0 }, path.segment(2).unwrap());
        assert_eq!(Veci { x: 3, y: 0 }, path.segment(3).unwrap());
        assert_eq!(Veci { x: 3, y: 1 }, path.segment(4).unwrap());
        assert_eq!(Veci { x: 2, y: 1 }, path.segment(5).unwrap());
        assert_eq!(Veci { x: 1, y: 1 }, path.segment(6).unwrap());
        assert_eq!(Veci { x: 1, y: 0 }, path.segment(7).unwrap());
        assert_eq!(Veci { x: 0, y: 0 }, path.segment(8).unwrap());
        assert_eq!(Veci { x: 0, y: 1 }, path.segment(9).unwrap());
    }
}
